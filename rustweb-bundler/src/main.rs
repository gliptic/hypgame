use std::env;
use std::path::{PathBuf, Path};
use std::time::{Instant, Duration};
use std::fs::{File, canonicalize};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::collections::{HashMap, HashSet, VecDeque};
use notify::{RecommendedWatcher, Watcher, RecursiveMode, Event};
use crossbeam_channel::{unbounded, Sender, Receiver, select, RecvError};
use rustweb_code::{hyp_parser as hyp};

fn print_line_at(data: &[u8], err: &hyp::ParseError, path: &Path) {
    let mut start = (err.0).0;
    while start > 0 && data[start - 1] != b'\n' {
        start -= 1;
    }
    let mut line = 1;
    let mut col = 1;
    for &d in &data[..(err.0).0] {
        if d == b'\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    let mut stop = (err.0).0;
    while stop < data.len() && data[stop + 1] != b'\n' {
        stop += 1;
    }

    println!("--> {}:{}:{}", &path.display(), line, col);
    println!(" | {}", std::str::from_utf8(&data[start..stop]).unwrap());
    println!(" {}", err.1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut deps = HashSet::new();
    let mut watching = HashSet::new();

    let (tx, rx) = unbounded();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1)).unwrap();

    'build_loop: loop {
        println!("building..");
        match build(&args[1], &args[2], &args[3]) {
            Ok(d) => {
                deps = d;
                for d in &deps {
                    let dir = d.parent().unwrap();
                    if !watching.contains(dir) {
                        watching.insert(dir.to_owned());
                        watcher.watch(dir, RecursiveMode::NonRecursive).unwrap();
                    }
                }
            },
            Err(_) => {
                // Just continue watching
            }
        }

        loop {
            select! {
                recv(rx) -> event => {
                    match event.unwrap() {
                        Ok(event) => {
                            println!("{:?}", event);
                            match event.kind {
                                notify::event::EventKind::Modify(_)
                                    if event.flag().is_none() && event.paths.len() == 1 => {

                                    let write_path = canonicalize(&event.paths[0]).unwrap();

                                    if deps.contains(&write_path) {
                                        continue 'build_loop;
                                    }
                                }
                                _ => {}
                            }
                        }
                        Err(_) => {
                        }
                    }
                }
            }
        }
    }
}

type DepSet = HashSet<PathBuf>;
type BuildResult = Result<DepSet, ()>; //Vec<rustweb_code::hyp::ParseError>>;

fn build(root_in: &str, js_out: &str, js_min_out: &str) -> BuildResult {

    let mut deps = HashSet::new();

    use rustweb_code::{
        hyp_resolver,
        hyp_to_js, hyp_to_glsl,
        js_bundler, glsl_bundler};

    let root = PathBuf::from(root_in);
    let mut have_read = HashMap::new();
    let mut to_read = VecDeque::new();
    let mut to_read_index = 0;

    to_read.push_front((hyp::Attr::None, root.clone()));
    have_read.insert(root, to_read_index);
    to_read_index += 1;

    let mut hyp_modules = Vec::new();

    while let Some((attr, path)) = to_read.pop_front() {
        println!("reading {:?}", &path);
        deps.insert(canonicalize(&path).unwrap());

        let mut data = std::fs::read(&path).unwrap();
        let hyp_module;
        if attr == hyp::Attr::None {
            data.push(0);

            let mut parser = hyp::Parser::new(data, path.clone());
            parser.next();
            hyp_module = match parser.rlambda_module() {
                Ok(hm) => hm,
                Err((data, path, err)) => {
                    print_line_at(&data, &err, &path);
                    //panic!("{}, at {}", err.1, (err.0).0);
                    return Err(());
                }
            };
        } else {
            // TODO: Make this easier
            hyp_module = hyp::Module {
                lambda: hyp::AstLambda {
                    params: Vec::new(),
                    param_locals: Vec::new(),
                    expr: Vec::new(),
                    return_type: hyp::AstType::None
                },
                src: data,
                path: path.clone(),
                uses: Vec::new(),
                locals: Vec::new(),
                exports: Vec::new(),
                exports_rev: HashMap::new(),
                language: hyp::Language::Binary
            };
        }

        let mut import_map = Vec::new();
        
        for hyp::Use(attr, rel) in &hyp_module.uses {
            let mut subpath = path.clone();
            subpath.pop();
            if rel.contains("/") {
                subpath = subpath.join(Path::new(&rel));
            } else {
                subpath.push(&rel);
            }
            if !rel.contains(".") {
                subpath.set_extension("hyp");
            }

            if !have_read.contains_key(&subpath) {
                have_read.insert(subpath.clone(), to_read_index);
                import_map.push(to_read_index);
                to_read.push_back((*attr, subpath));
                to_read_index += 1;
            } else {
                let index = *have_read.get(&subpath).unwrap();
                import_map.push(index);
            }
        }

        let module_name = path.file_stem().unwrap().to_string_lossy().into_owned();
        hyp_modules.push((module_name, hyp_module, import_map));
    }

    let mut module_infos = Vec::new();
    let mut module_lambdas = Vec::new();

    for (module_name, hyp_module, import_map) in hyp_modules {
        let (lambda, info) = hyp::ModuleInfo::from_module(module_name, import_map, hyp_module);
        module_infos.push(info);
        module_lambdas.push(lambda);
    }

    for i in 0..module_infos.len() {
        let mut resolver = hyp_resolver::Resolver::new(i, &mut module_infos);
        resolver.resolve_lambda(&mut module_lambdas[i]);
    }

    println!("success!");

    let mut seen_js = HashSet::new();

    for m in &mut module_infos {
        if m.language == hyp::Language::Js {
            for (_, local_name) in &mut m.locals {
                if seen_js.contains(local_name) {
                    //println!("name {} repeats", local_name);
                    // TODO: More modest renaming
                    for i in 0.. {
                        let new_name = format!("{}${}", local_name, i);
                        if !seen_js.contains(&new_name) {
                            *local_name = new_name.clone();
                            seen_js.insert(new_name);
                            break;
                        }
                    }
                } else {
                    seen_js.insert(local_name.clone());
                }
            }
        }
    }

    let mut bundler = js_bundler::JsBundler::new(&module_infos, &module_lambdas);
    
    let module_order = bundler.find_module_ordering();

    bundler.write_iife_begin_plain();

    // TODO: Combine together e.g. binaries and process as a unit

    for order_index in 0..module_order.len() {
        let module_index = module_order[order_index];

        println!("bundling #{}: {}", module_index, &module_infos[module_index as usize].name);

        let lang = module_infos[module_index as usize].language;

        if lang == hyp::Language::Js {
            let mut enc = hyp_to_js::JsEnc::new();
            enc.parse_hyp(&module_lambdas[module_index as usize].expr);
            if enc.errors.len() > 0 {
                for err in &enc.errors {
                    let mi = &module_infos[module_index as usize];
                    print_line_at(&mi.src, err, &mi.path);
                }
                //panic!("errors in hyp to js conversion");
                return Err(());
            }
            bundler.current_module = module_index as usize;
            bundler.stmts_inner_to_js(&enc.module.items);
        } else if lang == hyp::Language::Binary {
            bundler.current_module = module_index as usize;
            bundler.binary_to_js();
        }
    }

    bundler.write_iife_end();

    let js = &bundler.buf;

    let mut cl_proc = Command::new("java")
        .arg("-jar")
        .arg("closure-compiler.jar")
        .arg("--language_in=ES6")
        .arg("--language_out=ES6")
        .arg("--compilation_level")
        .arg("ADVANCED_OPTIMIZATIONS")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run closure-compiler");

    {
        let mut cl_in = cl_proc.stdin.as_mut().expect("failed to write to closure-compiler");
        cl_in.write_all(js.as_bytes()).expect("failed to write to closure-compiler");
    }

    {
        let mut out_file = File::create(js_out).unwrap();
        out_file.write_all(js.as_bytes()).expect("failed writing input file for closure-compiler");
    }

    let cl_out = cl_proc.wait_with_output().expect("failed to read from closure-compiler");

    {
        let mut out_min_file = File::create(js_min_out).unwrap();
        out_min_file.write_all(&cl_out.stdout).expect("failed writing minified js");
    }

    Ok(deps)
}
