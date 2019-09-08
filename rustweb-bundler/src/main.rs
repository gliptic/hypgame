#![feature(seek_convenience)]

use std::env;
use std::path::{PathBuf, Path};
use std::time::{Instant, Duration};
use std::fs::{File, canonicalize};
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::collections::{HashMap, HashSet, VecDeque};
use notify::{RecommendedWatcher, Watcher, RecursiveMode, Event};
use crossbeam_channel::{unbounded, Sender, Receiver, select, RecvError};
use rustweb_code::hyp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut deps = HashSet::new();
    let mut watching = HashSet::new();

    let (tx, rx) = unbounded();
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_millis(50)).unwrap();
    //let mut watcher: RecommendedWatcher = Watcher::new_immedit(tx, Duration::from_millis(300)).unwrap();

    'build_loop: loop {
        println!("building..");
        match build(&args[1], &args[2], &args[3], &args[4]) {
            Ok(d) => {
                deps = d;
            },
            Err(d) => {
                if deps.len() == 0 {
                    // Set dependencies if we don't have any
                    deps = d;
                }
            }
        }

        for d in &deps {
            let dir = d.parent().unwrap();
            if !watching.contains(dir) {
                watching.insert(dir.to_owned());
                watcher.watch(dir, RecursiveMode::NonRecursive).unwrap();
            }
        }

        let mut changes_seen_at: Option<Instant> = None;

        loop {
            select! {
                recv(rx) -> event => {
                    match event.unwrap() {
                        Ok(event) => {
                            // TODO: Do own debouncing to avoid building for
                            // every file save
                            println!("{:?}", event);
                            match event.kind {
                                notify::event::EventKind::Modify(_)
                                    if event.flag().is_none() && event.paths.len() == 1 => {

                                    let write_path = canonicalize(&event.paths[0]).unwrap();

                                    if deps.contains(&write_path) {
                                        changes_seen_at = Some(Instant::now());
                                    }
                                }
                                _ => {}
                            }
                        }
                        Err(_) => {
                        }
                    }
                }
                default(Duration::from_millis(100)) => {
                    if let Some(ch) = changes_seen_at {
                        if ch.elapsed() > Duration::from_millis(100) {
                            continue 'build_loop;
                        }
                    }
                }
            }
        }
    }
}

type DepSet = HashSet<PathBuf>;
type BuildResult = Result<DepSet, DepSet>; //Vec<rustweb_code::hyp::ParseError>>;
type HypModuleVec = Vec<(String, hyp::Module, Vec<u32>)>;

fn parse_modules(root_in: &str, deps: &mut DepSet)
    -> Result<HypModuleVec, ()> {

    let mut hyp_modules = Vec::new();

    {
        let root = PathBuf::from(root_in);
        let mut have_read = HashMap::new();
        let mut to_read = VecDeque::new();
        let mut to_read_index = 0;

        to_read.push_front((hyp::Attr::None, root.clone()));
        have_read.insert(root, to_read_index);
        to_read_index += 1;

        while let Some((attr, path)) = to_read.pop_front() {
            println!("reading {:?}", &path);
            deps.insert(canonicalize(&path).unwrap());

            let mut data = std::fs::read(&path).unwrap();
            let hyp_module;
            if attr == hyp::Attr::None {
                data.push(0);

                let mut parser = hyp::parser::Parser::new(data, path.clone());
                parser.next();
                hyp_module = match parser.rlambda_module() {
                    Ok(hm) => hm,
                    Err((data, path, err)) => {
                        hyp::print_line_at(&data, &err, &path);
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
                    local_types: vec![],
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
    }

    Ok(hyp_modules)
}

fn bundle_js(hyp_modules: HypModuleVec, debug: bool) -> String {
    use rustweb_code::{
        js_bundler
    };

    let mut module_infos = Vec::new();
    let mut module_lambdas = Vec::new();

    for (module_name, hyp_module, import_map) in hyp_modules {
        let (lambda, info) = hyp::ModuleInfo::from_module(module_name, import_map, hyp_module);
        module_infos.push(info);
        module_lambdas.push(lambda);
    }

    for i in 0..module_infos.len() {
        let mut resolver = hyp::resolver::Resolver::new(i, &mut module_infos, debug);
        resolver.resolve(&mut module_lambdas[i]);

        if resolver.errors.len() > 0 {
            for err in &resolver.errors {
                resolver.module_infos[i].print_line_at(err);
            }
            panic!("failed resolve");
        }
    }

    println!("success!");

    let mut bundler = js_bundler::JsBundler::new(&mut module_infos, &module_lambdas);

    bundler.run(debug);
    
    bundler.buf
}

fn build(root_in: &str, js_out: &str, js_min_out: &str, zip_min_out: &str) -> BuildResult {

    let mut deps = HashSet::new();

    use rustweb_code::{
        hyp_to_js, hyp_to_glsl,
        js_bundler, glsl_bundler};

    let hyp_modules = match parse_modules(root_in, &mut deps) {
        Ok(m) => m,
        Err(_) => return Err(deps)
    };

    let debug_js = &bundle_js(hyp_modules.clone(), true);
    let min_js = &bundle_js(hyp_modules, false);

    let mut cl_proc = Command::new("java")
        .arg("-jar")
        .arg("closure-compiler.jar")
        .arg("--language_in=ES6")
        .arg("--language_out=ES6")
        .arg("--compilation_level").arg("ADVANCED_OPTIMIZATIONS")
        //.arg("--jscomp_warning=reportUnknownTypes")
        //.arg("--warning_level=VERBOSE")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run closure-compiler");

    {
        let mut cl_in = cl_proc.stdin.as_mut().expect("failed to write to closure-compiler");
        cl_in.write_all(min_js.as_bytes()).expect("failed to write to closure-compiler");
    }

    {
        let mut out_file = File::create(js_out).unwrap();
        out_file.write_all(debug_js.as_bytes()).expect("failed writing non-minified output");
    }

    let cl_out = cl_proc.wait_with_output().expect("failed to read from closure-compiler");

    {
        let mut out_min_file = File::create(js_min_out).unwrap();
        out_min_file.write_all(&cl_out.stdout).expect("failed writing minified output");
    }

    {
        let mut w = File::create(zip_min_out).unwrap();
        let mut zip = zip::ZipWriter::new(w);
        let options = zip::write::FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
        zip.start_file("hyp.min.js", options).unwrap();
        zip.write(&cl_out.stdout).unwrap();
        w = zip.finish().unwrap();

        use std::io::Seek;
        println!("zip size: {}", w.stream_len().unwrap());
    }

    Ok(deps)
}
