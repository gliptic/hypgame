use bytes::{/*Bytes, BytesMut, */Buf, BufMut};
use std::io::Cursor;
//use crate::{JsAst, JsLit, JsOp, JsUnop, JsModule};
use crate::{GlslAst, GlslGlobal, GlslModule};
//use crate::glsl_bundler::{self, GlslBundler};
//use crate::js_bundler::{self, JsBundler};
//use crate::gimli;
//use gimli::{DebugAbbrev, DebugInfo, DebugLine, DebugStr, LittleEndian};

pub const SEC_CUSTOM: u8 = 0;
pub const SEC_TYPE: u8 = 1;
pub const SEC_IMPORT: u8 = 2;
pub const SEC_FUNCTION: u8 = 3;
pub const SEC_TABLE: u8 = 4;
pub const SEC_MEMORY: u8 = 5;
pub const SEC_GLOBAL: u8 = 6;
pub const SEC_EXPORT: u8 = 7;
pub const SEC_START: u8 = 8;
pub const SEC_ELEMENT: u8 = 9;
pub const SEC_CODE: u8 = 10;
pub const SEC_DATA: u8 = 11;

/*
Type 	1 	Function signature declarations
Import 	2 	Import declarations
Function 	3 	Function declarations
Table 	4 	Indirect function table and other tables
Memory 	5 	Memory attributes
Global 	6 	Global declarations
Export 	7 	Exports
Start 	8 	Start function declaration
Element 	9 	Elements section
Code 	10 	Function bodies (code)
Data 	11 	Data segments
*/

fn varuint32_len(mut v: u32) -> usize {
    let mut len = 0;
    loop {
        let more = v >= 0x80;
        len += 1;
        v >>= 7;
        if !more { break }
    }

    len
}

trait BufExt {
    fn get_varuint32(&mut self) -> u32;
}

trait BufMutExt {
    fn put_varuint32(&mut self, v: u32);
}

impl<T: Buf> BufExt for T {
    fn get_varuint32(&mut self) -> u32 {
        let mut v = 0;
        let mut offset = 0;
        
        loop {
            let piece = self.get_u8();
            v |= ((piece & 0x7f) as u32) << offset;
            if piece < 0x80 { break }
            offset += 7;
        }

        v
    }
}

impl<T: BufMut> BufMutExt for T {
    fn put_varuint32(&mut self, mut v: u32) {

        loop {
            let piece = v & 0x7f;
            let more = v >= 0x80;
            self.put_u8((piece as u8) + (if more { 0x80 } else { 0x00 }));
            v >>= 7;
            if !more { break }
        }
    }
}

pub struct Section {
    name: Vec<u8>,
    data: Vec<u8>,
    id: u8,
}

impl Section {
    pub fn write(&self, buf: &mut Vec<u8>) {
        let payload_len = if self.id == 0 {
            self.data.len() + varuint32_len(self.name.len() as u32) + self.name.len()
        } else {
            self.data.len()
        };

        buf.put_u8(self.id);
        buf.put_varuint32(payload_len as u32);

        if self.id == 0 {
            buf.put_varuint32(self.name.len() as u32);
            buf.put_slice(&self.name);
        }

        buf.put_slice(&self.data);
    }
}

pub struct Reader {
    magic: u32,
    version: u32,
    sections: Vec<Section>
}

impl Reader {
    pub fn from(data: &[u8]) -> Reader {
        let mut buf = Cursor::new(data);
        let magic = buf.get_u32_le();
        let version = buf.get_u32_le();

        let mut sections = Vec::new();

        while buf.remaining() > 0 {
            let id = buf.get_u8();
            let mut payload_len = buf.get_varuint32() as usize;

            let mut name;
            if id == 0 {
                let p = buf.position() as usize;
                let name_len = buf.get_varuint32() as usize;
                //println!("Pos: {:x}. Name len next {}", buf.position(), name_len);
                name = vec![0u8; name_len];
                buf.copy_to_slice(&mut name[..]);
                payload_len -= buf.position() as usize - p;
            } else {
                //println!("Pos: {:x}", buf.position());
                name = Vec::new();
            }

            //println!("Section ID {}, size {:x}", id, payload_len);

            let mut data = vec![0u8; payload_len];
            buf.copy_to_slice(&mut data[..]);

            sections.push(Section { name, data, id });
        }

        Reader {
            magic,
            version,
            sections
        }
    }

    pub fn bundle(&self) -> (String, Vec<u8>) {
/*
        let mut wasm_stripped = Vec::new();
        let mut bundle = JsBundler::new();
        let mut modules = Vec::new();
        let mut glsl_modules = Vec::new();
        let mut glsl_bun = GlslBundler::new();

        wasm_stripped.put_u32_le(self.magic);
        wasm_stripped.put_u32_le(self.version);

        let mut wasm_imports = Vec::new();
        let mut debug_info_data = Vec::new();
        let mut debug_abbrev_data = Vec::new();
        let mut debug_str_data = Vec::new();

        for s in &self.sections {
            if s.id == 0 {
                if &s.name == b".debug_info" {
                    println!("Found .debug_info");
                    debug_info_data = s.data.to_vec();
                } else if &s.name == b".debug_abbrev" {
                    println!("Found .debug_abbrev");
                    debug_abbrev_data = s.data.to_vec();
                } else if &s.name == b".debug_str" {
                    println!("Found .debug_str");
                    debug_str_data = s.data.to_vec();
                } else if &s.name == b"js" {
                    let mut reader = Cursor::new(&s.data[..]);

                    while reader.remaining() > 0 {
                        let mut module: JsModule = bincode::deserialize_from(&mut reader).unwrap();

                        let mut module_info = js_bundler::ModuleInfo {
                            name: module.name.clone(),
                            export_names: Vec::new(),
                            mapped_names: module.exports.iter()
                                .map(|n| format!("{}_{}", &module.name, n)).collect(),
                            import_map: Vec::new()
                        };

                        // Move export names into module_info
                        std::mem::swap(&mut module_info.export_names, &mut module.exports);

                        for item in &module.items {
                            match item {
                                &JsAst::Fn { index, exported, .. } => {
                                    if exported {
                                        wasm_imports.push(module_info.mapped_names[index as usize].clone());
                                    }
                                },
                                _ => {}
                            }
                        }

                        //println!("export names in {}: {:?}", &module_info.name, &module_info.export_names);
                        
                        bundle.module_infos.push(module_info);
                        modules.push(module);
                    }

                    //dbg!(bundle.module_infos.len());

                } else if &s.name == b"glsl" {
                    let mut reader = Cursor::new(&s.data[..]);

                    while reader.remaining() > 0 {
                        let mut module: GlslModule = bincode::deserialize_from(&mut reader).unwrap();
                        
                        glsl_bun.module_infos.push(glsl_bundler::ModuleInfo {
                            mapped_names: module.locals.iter().map(|(_, name)| name.clone()).collect(),
                            local_types: module.locals.iter().map(|(ty, _)| ty.clone()).collect(),
                        });

                        glsl_modules.push(module);
                    }
                }
            } else {
                s.write(&mut wasm_stripped);
            }
        }

        //
        let debug_info = &DebugInfo::new(&debug_info_data[..], LittleEndian);
        let debug_abbrev = &DebugAbbrev::new(&debug_abbrev_data[..], LittleEndian);
        let debug_str = &DebugStr::new(&debug_str_data[..], LittleEndian);
        let mut iter = debug_info.units();
        while let Some(unit) = iter.next().unwrap() {
            //println!("unit's length is {}", unit.unit_length());

            let abbrevs = unit.abbreviations(debug_abbrev).unwrap();
            let mut cursor = unit.entries(&abbrevs);
            //while let Ok(_) = cursor.next_
            cursor.next_dfs().expect("???");

            //let root = cursor.current().expect("missing die");
            let mut depth = 0;
            while let Some((delta_depth, current)) = cursor.next_dfs().expect("Should parse next dfs") {
                // Update depth value, and break out of the loop when we
                // return to the original starting position.
                depth += delta_depth;
                if depth <= 0 {
                    break;
                }

                match current.tag() {
                    gimli::DW_TAG_variable => {
                        let comp_name = current.attr(gimli::DW_AT_name)
                            .unwrap()
                            .and_then(|attr| attr.string_value(debug_str).and_then(|s| std::str::from_utf8(s.slice()).ok()));

                        if let Some(name) = comp_name {

                            let loc = current.attr(gimli::DW_AT_location)
                                .ok()
                                .and_then(|x| x)
                                .and_then(|a| a.exprloc_value());

                            if let Some(loc) = loc {
                                let mut eval = loc.evaluation(unit.encoding());
                                let res = eval.evaluate().unwrap();
                                if let gimli::read::EvaluationResult::RequiresRelocatedAddress(addr) = res {
                                    println!("found var: {} -> {}", name, addr);
                                    bundle.wasm_offsets.insert(name.to_owned(), addr);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        //

        for (module_index, m) in modules.iter().enumerate() {
            for i in 0..m.imports.len() {
                let import_name = &m.imports[i];
                let mut found = false;
                for j in 0..bundle.module_infos.len() {
                    if &bundle.module_infos[j].name == import_name {
                        found = true;
                        println!("import module in {}-{}: {}", module_index, &m.name, j as u32);
                        bundle.module_infos[module_index].import_map.push(j as u32);
                        break;
                    }
                }

                if !found {
                    panic!("module {} not found", &import_name);
                }
            }
        }

        bundle.glsl_collection = Some(GlslCollection(glsl_bun, glsl_modules));

        let module_order = bundle.find_module_ordering();

        println!("wasm stripped: {}", wasm_stripped.len());

        bundle.write_iife_begin(&wasm_stripped, &wasm_imports);

        for order_index in 0..module_order.len() {
            let module_index = module_order[order_index];
            bundle.current_module = module_index as usize;
            let m = &modules[module_index as usize];

            bundle.stmts_inner_to_js(&m.items);
        }

        bundle.write_iife_end();

        //println!("{}", &bundle.buf);
        (bundle.buf, wasm_stripped)
        //"".to_owned()
        */
        (String::new(), Vec::new())
    }
}