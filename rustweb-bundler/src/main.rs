use std::env;
use std::path::{PathBuf};
use std::fs::File;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::collections::{HashMap, HashSet, VecDeque};

fn print_line_at(data: &[u8], at: usize) {
    let mut start = at;
    while start > 0 && data[start - 1] != b'\n' {
        start -= 1;
    }
    let mut stop = at;
    while stop < data.len() && data[stop + 1] != b'\n' {
        stop += 1;
    }

    println!("line: {}", std::str::from_utf8(&data[start..stop]).unwrap());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if false {

        let opt_filename = format!("{}.opt.wasm", &args[1]);
        let _output = Command::new("wasm-opt")
            .arg(&args[1])
            .arg("--flatten")
            .arg("-Oz")
            .arg("-o")
            .arg(&opt_filename)
            .output()
            .expect("failed to run wasm-opt");

        let mut file = File::open(&opt_filename).unwrap();

        let mut data = Vec::new();
        file.read_to_end(&mut data).unwrap();
        //let mut data = output.stdout;

        println!("got back: {}", data.len());

        let reader = rustweb_code::wasm::Reader::from(&data);

        let (js, wasm_stripped) = reader.bundle();

        //println!("unoptimized:\n{}", &js);

        let mut cl_proc = Command::new("java")
            .arg("-jar")
            .arg("closure-compiler.jar")
            .arg("--language_in=ES6")
            .arg("--language_out=ES6")
            .arg("--compilation_level")
            .arg("SIMPLE_OPTIMIZATIONS")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to run closure-compiler");

        {
            let mut cl_in = cl_proc.stdin.as_mut().expect("failed to write to closure-compiler");
            cl_in.write_all(js.as_bytes()).expect("failed to write to closure-compiler");
        }

        {
            let mut out_file = File::create(&args[2]).unwrap();
            out_file.write_all(js.as_bytes()).expect("failed writing input file for closure-compiler");
        }

        {
            let mut out_file = File::create(format!("{}.stripped.wasm", &args[1])).unwrap();
            out_file.write_all(&wasm_stripped[..]).expect("failed writing stripped wasm");
        }
        

        let cl_out = cl_proc.wait_with_output().expect("failed to read from closure-compiler");

        {
            let mut out_min_file = File::create(&args[3]).unwrap();
            out_min_file.write_all(&cl_out.stdout).expect("failed writing minified js");
        }
        //println!("optimized:\n{}", String::from_utf8_lossy(&cl_out.stdout));

        //java -jar closure-compiler.jar --language_in=ES6 --language_out=ES6 --js_output_file=pkg/bundle.min.js --externs www/externs.js --compilation_level SIMPLE_OPTIMIZATIONS pkg/bundle.js
    } else {
        use rustweb_code::{
            hyp_parser, hyp_resolver,
            hyp_to_js, hyp_to_glsl,
            js_bundler, glsl_bundler};

        let root = PathBuf::from(&args[1]);
        let mut have_read = HashMap::new();
        let mut to_read = VecDeque::new();
        let mut to_read_index = 0;

        to_read.push_front(root.clone());
        have_read.insert(root, to_read_index);
        to_read_index += 1;

        //let mut modules = Vec::new();

        let mut hyp_modules = Vec::new();

        while let Some(path) = to_read.pop_front() {
            println!("reading {:?}", &path);
            let mut data = std::fs::read(&path).unwrap();
            data.push(0);
            let mut parser = hyp_parser::Parser::new(&data);
            parser.next();
            let hyp_module = match parser.rlambda_module() {
                Ok(hm) => hm,
                Err(err) => {
                    print_line_at(&data, err.0);
                    panic!("{}, at {}", err.1, err.0);
                }
            };

            let mut import_map = Vec::new();
            
            for u in &hyp_module.uses {
                let mut subpath = path.clone();
                subpath.pop();
                subpath.push(&u);
                subpath.set_extension("hyp");

                if !have_read.contains_key(&subpath) {
                    have_read.insert(subpath.clone(), to_read_index);
                    import_map.push(to_read_index);
                    to_read.push_back(subpath);
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
            let (lambda, info) = hyp_parser::ModuleInfo::from_module(module_name, import_map, hyp_module);
            module_infos.push(info);
            module_lambdas.push(lambda);
        }

        for i in 0..module_infos.len() {
            let mut resolver = hyp_resolver::Resolver::new(i, &mut module_infos);
            resolver.resolve_lambda(&mut module_lambdas[i]);
        }

        println!("success!");

        // TODO: Rename symbol that could clash in module_infos

        let mut seen_js = HashSet::new();

        for m in &mut module_infos {
            if m.language == hyp_parser::Language::Js {
                for (_, local_name) in &mut m.locals {
                    if seen_js.contains(local_name) {
                        println!("name {} repeats", local_name);
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

        for order_index in 0..module_order.len() {
            let module_index = module_order[order_index];

            println!("bundling #{}: {}", module_index, &module_infos[module_index as usize].name);

            if module_infos[module_index as usize].language == hyp_parser::Language::Js {
                let mut enc = hyp_to_js::JsEnc::new();
                enc.parse_hyp(&module_lambdas[module_index as usize].expr);
                bundler.current_module = module_index as usize;
                bundler.stmts_inner_to_js(&enc.module.items);
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
            .arg("SIMPLE_OPTIMIZATIONS")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to run closure-compiler");

        {
            let mut cl_in = cl_proc.stdin.as_mut().expect("failed to write to closure-compiler");
            cl_in.write_all(js.as_bytes()).expect("failed to write to closure-compiler");
        }

        {
            let mut out_file = File::create(&args[2]).unwrap();
            out_file.write_all(js.as_bytes()).expect("failed writing input file for closure-compiler");
        }

        let cl_out = cl_proc.wait_with_output().expect("failed to read from closure-compiler");

        {
            let mut out_min_file = File::create(&args[3]).unwrap();
            out_min_file.write_all(&cl_out.stdout).expect("failed writing minified js");
        }

        //println!("{}", &bundler.buf);
    }
}
