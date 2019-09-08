use std::collections::{HashMap};
use std::path::{PathBuf};
use std::rc::Rc;

use super::*;

const LEX_DIGIT: u16 = 1 << 0;
const LEX_BEG_IDENT: u16 = 1 << 1;
const LEX_INNER_IDENT: u16 = 1 << 2;
const LEX_WHITESPACE: u16 = 1 << 3;
const LEX_OP: u16 = 1 << 4;
const LEX_SINGLE_CHAR: u16 = 1 << 5;
const LEX_NEWLINE: u16 = 1 << 6;
const LEX_STR: u16 = 1 << 7;

const TT_COMMA: u16 = 0;
const TT_COLON: u16 = 1;

const TT_DOT: u16 = 2;
//const TT_BAR: u16 = 3;
const TT_UNDERSCORE: u16 = 4;
const TT_IDENT: u16 = 5;
const TT_OPIDENT: u16 = 6;
const TT_CONSTINT: u16 = 7;
const TT_LBRACE: u16 = 8;
const TT_RBRACE: u16 = 9;
const TT_LPAREN: u16 = 10;
const TT_RPAREN: u16 = 11;
const TT_LBRACKET: u16 = 12;
const TT_RBRACKET: u16 = 13;
const TT_EQUAL: u16 = 14;
const TT_QUESTIONMARK: u16 = 15;
const TT_BACKSLASH: u16 = 16;
const TT_DOLLAR: u16 = 17;
const TT_LET: u16 = 18;
const TT_VAR: u16 = 19;
const TT_FUN: u16 = 20;
const TT_TYP: u16 = 21;
const TT_AS: u16 = 22;
const TT_WHILE: u16 = 22;
const TT_LOOP: u16 = 23;
const TT_FOR: u16 = 24;
const TT_RETURN: u16 = 25;
const TT_IF: u16 = 26;
const TT_ELSE: u16 = 26;
const TT_USE: u16 = 27;
const TT_CONSTSTR: u16 = 28;
const TT_PUB: u16 = 29;
const TT_AT: u16 = 30;
const TT_CONSTFLOAT: u16 = 31;
const TT_SEMICOLON: u16 = 32;
const TT_NEW: u16 = 33;
const TT_BREAK: u16 = 34;
const TT_IN: u16 = 35;
const TT_DOUBLEDOT: u16 = 36;
const TT_TYPE: u16 = 37;
const TT_EOF: u16 = 254;
const TT_INVALID: u16 = 255;

type Tt = u16;

const fn ltab_pair(a: u16, b: u16) -> u16 {
    a | (b << 8)
}

const fn ltab_pair_rtl(a: u16, b: u16) -> u16 {
    ltab_pair(a, b | 0x80)
}

pub struct Parser {
    lextable: [u16; 256],
    tt: u16,
    src: Vec<u8>,
    prev_cur: usize,
    cur: usize,
    token_ident: Ident,
    token_prec: u8,
    is_rtl: bool,
    token_number: i64,
    token_float: f64,
    //module: Module,
    uses: Vec<Use>,
    locals: Vec<LocalDef>,
    local_types: Vec<TypeDef>,
    exports: Vec<u32>, // indexes into locals
    exports_rev: HashMap<Ident, u32>,
    language: Language,
    path: PathBuf
}

impl Parser {
    
    pub fn new(src: Vec<u8>, path: PathBuf) -> Parser {
        let mut lextable = [ltab_pair(LEX_SINGLE_CHAR, TT_INVALID); 256];

        lextable[0] = ltab_pair(LEX_SINGLE_CHAR, TT_EOF);

        for c in b'a'..=b'z' {
            lextable[c as usize] = LEX_BEG_IDENT | LEX_INNER_IDENT;
        }

        for c in b'A'..=b'Z' {
            lextable[c as usize] = LEX_BEG_IDENT | LEX_INNER_IDENT;
        }

        for c in b'0'..=b'9' {
            lextable[c as usize] = LEX_DIGIT | LEX_INNER_IDENT;
        }

        let charmaps = [
            (b'\'', LEX_BEG_IDENT),
            (b'_', LEX_INNER_IDENT),
            (b' ', LEX_WHITESPACE),
            (b'\t', LEX_WHITESPACE),
            (b'\\', LEX_WHITESPACE),
            (b'\r', LEX_WHITESPACE | LEX_NEWLINE),
            (b'\n', LEX_WHITESPACE | LEX_NEWLINE),
            (b'#', LEX_WHITESPACE),
            (b'"', LEX_STR),
            (b'{', ltab_pair(LEX_SINGLE_CHAR, TT_LBRACE)),
            (b'}', ltab_pair(LEX_SINGLE_CHAR, TT_RBRACE)),
            (b'(', ltab_pair(LEX_SINGLE_CHAR, TT_LPAREN)),
            (b')', ltab_pair(LEX_SINGLE_CHAR, TT_RPAREN)),
            (b'[', ltab_pair(LEX_SINGLE_CHAR, TT_LBRACKET)),
            (b']', ltab_pair(LEX_SINGLE_CHAR, TT_RBRACKET)),
            (b',', ltab_pair(LEX_SINGLE_CHAR, TT_COMMA)),
            (b';', ltab_pair(LEX_SINGLE_CHAR, TT_SEMICOLON)),
            //(b'.', ltab_pair(LEX_SINGLE_CHAR, TT_DOT)),
            //(b'?', ltab_pair(LEX_SINGLE_CHAR, TT_QUESTIONMARK)),
            (b':', ltab_pair(LEX_OP, 0)),
            (b'|', ltab_pair(LEX_OP, 1)),
            (b'@', ltab_pair(LEX_OP, 2)),
            (b'&', ltab_pair(LEX_OP, 2)),
            (b'^', ltab_pair(LEX_OP, 2)),
            (b'!', ltab_pair(LEX_OP, 3)),
            (b'=', ltab_pair(LEX_OP, 3)),
            (b'<', ltab_pair(LEX_OP, 3)),
            (b'>', ltab_pair(LEX_OP, 3)),
            (b'+', ltab_pair(LEX_OP, 4)),
            (b'-', ltab_pair(LEX_OP, 4)),
            (b'*', ltab_pair(LEX_OP, 5)),
            (b'%', ltab_pair(LEX_OP, 5)),
            (b'/', ltab_pair(LEX_OP, 5)),
            (b'~', ltab_pair(LEX_OP, 5)),
            (b'.', ltab_pair(LEX_OP, 6)),
        ];

        for &(c, lt) in &charmaps[..] {
            lextable[c as usize] = lt;
        }

        Parser {
            lextable,
            src,
            tt: TT_LBRACE,
            prev_cur: 0,
            cur: 0,
            token_number: 0,
            token_float: 0.0,
            token_ident: String::new(),
            token_prec: 0,
            is_rtl: false,
            uses: Vec::new(),
            locals: Vec::new(),
            local_types: vec![],
            exports: Vec::new(),
            exports_rev: HashMap::new(),
            language: Language::Js,
            path
        }
    }

    pub fn next(&mut self) {
        self.prev_cur = self.cur;
        let mut ch = self.src[self.cur];
        self.cur += 1;
        let mut lexcat = self.lextable[ch as usize];
        let tt;

        if lexcat & (LEX_OP | LEX_BEG_IDENT) != 0 {
            let cont_mask = if (lexcat & LEX_OP) != 0 { LEX_OP } else { LEX_INNER_IDENT };
            let ident_beg = self.cur - 1;

            loop {
                ch = self.src[self.cur];
                if (self.lextable[ch as usize] & cont_mask) == 0 {
                    break;
                }
                self.cur += 1;
            }

            let ident_end = self.cur;

            //println!("{:?}", std::str::from_utf8(&self.src[ident_beg..ident_end]));

            let ident_slice = &self.src[ident_beg..ident_end];
            tt = match ident_slice {
                //b"|" => TT_BAR,
                b"=" => TT_EQUAL,
                b"." => TT_DOT,
                b".." => TT_DOUBLEDOT,
                b":" => TT_COLON,
                b"@" => TT_AT, // TODO: Leave as operator?
                b"let" => TT_LET,
                b"pub" => TT_PUB,
                b"var" => TT_VAR,
                b"fn" => TT_FUN,
                b"typ" => TT_TYP,
                b"as" => TT_AS,
                b"while" => TT_WHILE,
                b"loop" => TT_LOOP,
                b"for" => TT_FOR,
                b"in" => TT_IN,
                b"return" => TT_RETURN,
                b"if" => TT_IF,
                b"else" => TT_ELSE,
                b"use" => TT_USE,
                b"new" => TT_NEW,
                b"type" => TT_TYPE,
                b"break" => TT_BREAK,
                _ => {
                    self.token_ident = String::from_utf8_lossy(&self.src[ident_beg..ident_end]).into_owned();
                    let prec_data = (lexcat >> 8) as u8;
                    
                    if cont_mask == LEX_OP {
                        if ident_slice == b":="
                        || ident_slice == b"+="
                        || ident_slice == b"-="
                        || ident_slice == b"*="
                        || ident_slice == b"/="
                        || ident_slice == b"%=" {
                            self.token_prec = 0;
                            self.is_rtl = true;
                        } else {
                            self.token_prec = prec_data & 0x7f;
                            self.is_rtl = prec_data >= 0x80;
                        }
                        TT_OPIDENT
                    } else {
                        TT_IDENT
                    }
                }
            };
        } else if (lexcat & LEX_SINGLE_CHAR) != 0 {
            tt = lexcat >> 8;
        } else if (lexcat & LEX_WHITESPACE) != 0 {
            loop {
                if ch == b'#' {
                    let mut nest = if self.src[self.cur] == b'[' {
                        self.cur += 1;
                        1
                    } else {
                        0
                    };

                    loop {
                        ch = self.src[self.cur];
                        self.cur += 1;

                        if nest > 0 {
                            if ch == b'[' {
                                nest += 1;
                            } else if ch == b']' {
                                nest -= 1;
                                if nest == 0 {
                                    break;
                                }
                            }
                        }

                        lexcat = self.lextable[ch as usize];
                        if nest == 0 && ((lexcat & LEX_NEWLINE) != 0 || ch == 0) {
                            break;
                        }
                    }
                }

                if (lexcat & LEX_NEWLINE) != 0
                 && self.tt != TT_LPAREN
                 && self.tt != TT_LBRACKET
                 && self.tt != TT_LBRACE
                 && self.tt != TT_COMMA
                 && self.tt != TT_DOT
                 && self.tt != TT_DOUBLEDOT
                 && self.tt != TT_SEMICOLON
                 && self.tt != TT_EQUAL
                 //&& self.tt != TT_BAR
                 && self.tt != TT_OPIDENT
                 && self.tt != TT_TYP
                 && self.tt != TT_FUN
                 && self.tt != TT_LET
                 && self.tt != TT_TYPE
                 && self.tt != TT_DOLLAR {

                    self.tt = TT_SEMICOLON;
                    //dbg!(self.tt);
                    return;
                }

                if ch == b'$' {
                    self.tt = TT_DOLLAR;
                }
                ch = self.src[self.cur];
                lexcat = self.lextable[ch as usize];

                if (lexcat & LEX_WHITESPACE) == 0 {
                    break;
                }

                self.cur += 1;
            }

            return self.next();
        } else if (lexcat & LEX_DIGIT) != 0 {
            
            if ch == b'0' && self.src[self.cur] == b'x' {
                self.cur += 1;
                let hex_start = self.cur;
                loop {
                    ch = self.src[self.cur];
                    if !((ch >= b'0' && ch <= b'9')
                      || (ch >= b'a' && ch <= b'f')
                      || (ch >= b'A' && ch <= b'F')) {
                        break;
                    }
                    self.cur += 1;
                }

                let end = self.cur;
                self.token_number = i64::from_str_radix(
                        std::str::from_utf8(&self.src[hex_start..end]).unwrap(), 16).unwrap();
                tt = TT_CONSTINT;
            } else {
                let start = self.cur - 1;

                loop {
                    ch = self.src[self.cur];
                    lexcat = self.lextable[ch as usize];
                    if (lexcat & LEX_DIGIT) == 0 {
                        break;
                    }
                    self.cur += 1;
                }

                // TODO: Require a digit after . for disambiguation
                if ch == b'.'
                && self.src[self.cur + 1] != b'.' {

                    self.cur += 1;
                    loop {
                        ch = self.src[self.cur];
                        lexcat = self.lextable[ch as usize];
                        if (lexcat & LEX_DIGIT) == 0 {
                            break;
                        }
                        self.cur += 1;
                    }

                    let end = self.cur;
                    self.token_float = std::str::from_utf8(&self.src[start..end]).unwrap().parse::<f64>().unwrap();
                    tt = TT_CONSTFLOAT;
                } else {
                    let end = self.cur;
                    self.token_number = std::str::from_utf8(&self.src[start..end]).unwrap().parse::<i64>().unwrap();
                    tt = TT_CONSTINT;
                }
            }

            
        } else if (lexcat & LEX_STR) != 0 {
            self.token_ident.clear();

            loop {
                ch = self.src[self.cur];
                if ch == b'"' || ch == 0 {
                    break;
                }
                // TODO: Handle UTF-8 properly
                self.token_ident.push(ch as char);
                self.cur += 1;
            }

            if ch == b'"' {
                self.cur += 1;
            }

            tt = TT_CONSTSTR;
        } else {
            panic!("Unexpected character {}", ch as char);
        }

        //dbg!(tt);

        self.tt = tt;
    }

    pub fn test(&mut self, token: Tt) -> bool {
        if token == self.tt {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn peek_check(&mut self, token: Tt) -> ParseResult<()> {
        if token != self.tt {
            println!("Expected {}, got {}", token, self.tt);
            Err(ParseError(self.span(), "unexpected token"))
        } else {
            Ok(())
        }
    }

    pub fn check(&mut self, token: Tt) -> ParseResult<()> {
        if token != self.tt {
            println!("Expected {}, got {}", token, self.tt);
            Err(ParseError(self.span(), "unexpected token"))
        } else {
            self.next();
            Ok(())
        }
    }

    pub fn check_ident(&mut self) -> ParseResult<Ident> {
        if self.tt == TT_IDENT || self.tt == TT_OPIDENT {
            let ident = std::mem::replace(&mut self.token_ident, Ident::new());
            self.next();
            Ok(ident)
        } else {
            Err(ParseError(self.span(), "expected identifier"))
        }
    }

    pub fn rtype(&mut self) -> ParseResult<AstType> {
        if self.tt == TT_OPIDENT {
            match &self.token_ident[..] {
                "&" => {
                    self.next();
                    Ok(AstType::Ptr(Box::new(self.rtype()?)))
                }
                _ => Err(ParseError(self.span(), "unexpected type operator"))
            }
        } else if self.test(TT_LBRACE) {
            let mut fields = Vec::new();
            
            while self.tt != TT_RBRACE && self.tt != TT_EOF {
                let name = self.check_ident()?;
                self.check(TT_COLON)?;
                let ty = self.rtype()?;
                
                fields.push(StructField::new(name, ty));

                if !self.test_comma_or_semi() {
                    break;
                }
            }

            self.check(TT_RBRACE)?;

            let struct_def = StructDef::new(fields);

            Ok(AstType::MemStruct(Rc::new(struct_def)))
        } else {
            let name = self.check_ident()?;
            Ok(AstType::Other(name))
        }
    }

    pub fn span(&self) -> Span {
        Span(self.prev_cur, self.cur)
    }

    pub fn new_app(&mut self, fun: AstRef, params: Vec<Ast>, kind: AppKind) -> Ast {
        Ast { loc: self.span(), attr: Attr::None, ty: AstType::Any, data: AstData::App { fun, params, kind } }
    }

    pub fn new_lambda(&mut self, params: Vec<ParamDef>, param_locals: Vec<u32>, return_type: AstType) -> AstLambda {
        AstLambda {
            params,
            param_locals,
            expr: Vec::new(),
            return_type
        }
    }

    pub fn test_comma_or_semi(&mut self) -> bool {
        self.test(TT_COMMA) || self.test(TT_SEMICOLON)
    }

    pub fn rpattern(&mut self, locals: &mut Vec<u32>, parent_ty: &AstType) -> ParseResult<Pattern> {
        if self.tt == TT_IDENT {
            let name = self.check_ident()?;
            // TODO: Mutable pattern var
            let local_index = self.new_local(name.clone(), parent_ty.clone(), false);
            locals.push(local_index);
            Ok(Pattern::Local(local_index))
        } else if self.test(TT_LBRACKET) {
            let mut subpat = Vec::new();
            while self.tt != TT_RBRACKET {
                // TODO: What type? Types should probably be
                // decided in resolve by deconstructing the pattern.
                subpat.push(self.rpattern(locals, &AstType::Any)?);
                if !self.test_comma_or_semi() {
                    break;
                }
            }

            self.next();

            Ok(Pattern::Array(subpat))
        } else {
            Err(ParseError(self.span(), "invalid pattern"))
        }
    }

    pub fn rparameters(&mut self, params: &mut Vec<ParamDef>, locals: &mut Vec<u32>) -> ParseResult<()> {
        while self.tt != TT_RPAREN {
            //let name = self.check_ident()?;
            let pat = self.rpattern(locals, &AstType::Any)?;
            let ty;
            if self.test(TT_COLON) {
                ty = self.rtype()?;
            } else {
                ty = AstType::Any;
            }

            //let local_index = self.new_local(name.clone(), ty.clone());
            // TODO: Don't need to store name, ty here later on
            params.push(ParamDef { ty, pat });
            //locals.push(local_index);
            if !self.test_comma_or_semi() {
                break;
            }
        }

        Ok(())
    }

    pub fn test_double_bar(&mut self) -> bool {
        if self.tt == TT_OPIDENT && &self.token_ident == "||" {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn test_bar(&mut self) -> bool {
        if self.tt == TT_OPIDENT && &self.token_ident == "|" {
            self.next();
            true
        } else {
            false
        }
    }

    pub fn check_bar(&mut self) -> ParseResult<()> {
        if !self.test_bar() {
            Err(ParseError(self.span(), "Expected '|'"))
        } else {
            Ok(())
        }
    }

    pub fn rlambda_block_del(&mut self) -> ParseResult<AstLambda> {
        self.check(TT_LBRACE)?;

        let mut params = Vec::new();
        let mut locals = Vec::new();
        
        if self.test_double_bar() {
            // Do nothing. Why support this?
        } else if self.test_bar() {
            self.rparameters(&mut params, &mut locals)?;
            self.check_bar()?;
        }

        let mut lambda = self.new_lambda(params, locals, AstType::Any);
        self.rlambda(&mut lambda, TT_RBRACE)?;

        Ok(lambda)
    }

    pub fn rprimary_del(&mut self) -> ParseResult<Ast> {
        let r = match self.tt {
            TT_CONSTINT => {
                Ast { loc: self.span(), attr: Attr::None, ty: AstType::U64,
                    data: AstData::ConstLit { v: Lit::Int(self.token_number) } }
            },
            TT_CONSTFLOAT => {
                Ast { loc: self.span(), attr: Attr::None, ty: AstType::F64,
                    data: AstData::ConstLit { v: Lit::Float(self.token_float) } }
            },
            TT_CONSTSTR => {
                Ast {
                    loc: self.span(),
                    attr: Attr::None,
                    ty: AstType::Str,
                    data: AstData::ConstStr {
                        v: std::mem::replace(&mut self.token_ident, String::new())
                    }
                }
            },
            TT_NEW => {
                self.next();
                if self.test(TT_LBRACE) {
                    let mut assignments = Vec::new();
                    while self.tt != TT_RBRACE && self.tt != TT_EOF {
                        let name = self.check_ident()?;
                        let v;
                        if self.test(TT_COLON) {
                            v = self.rexpr()?;
                        } else {
                            v = self.ast_anyty(AstData::Ident {
                                s: name.clone()
                            });
                        }

                        assignments.push((name, v));

                        if !self.test_comma_or_semi() {
                            break;
                        }
                    }

                    self.ast_anyty(AstData::NewObject { assignments })
                } else {
                    let ctor = self.rprimary_del()?;
                    self.next();

                    let mut params = Vec::new();
                    self.check(TT_LPAREN)?;
                        
                    self.rrecord_body(&mut params)?;
                    self.peek_check(TT_RPAREN)?;

                    self.ast_anyty(AstData::NewCtor {
                        ctor: Box::new(ctor),
                        params
                    })
                }
            }
            TT_IDENT => {
                //self.find_val_local(self.token_ident)
                let ident = std::mem::replace(&mut self.token_ident, Ident::new());
                self.ast_anyty(AstData::Ident { s: ident })
            }
            TT_LPAREN => {
                self.next();
                let v = self.rexpr()?;
                self.peek_check(TT_RPAREN)?;
                v
            }
            TT_LBRACE => {
                let lambda = self.rlambda_block_del()?;
                self.ast_anyty(AstData::Lambda { lambda })
            }
            TT_LBRACKET => {
                self.next();
                let mut elems = Vec::new();
                self.rrecord_body(&mut elems)?;
                self.ast_anyty(AstData::Array { elems })
            }
            _ => { return Err(ParseError(self.span(), "expected expression")) }
        };

        Ok(r)
    }

    pub fn rrecord_body(&mut self, exprs: &mut Vec<Ast>) -> ParseResult<()> {
        while self.tt != TT_RBRACE && self.tt != TT_RPAREN && self.tt != TT_RBRACKET && self.tt != TT_EOF {
            let v = self.rexpr()?;
            exprs.push(v);

            if !self.test_comma_or_semi() {
                break;
            }
        }

        Ok(())
    }

    pub fn rsimple_expr_tail(&mut self, mut r: Ast) -> ParseResult<Ast> {
        loop {
            if self.test(TT_DOT) {
                let range = false; // TODO: Use TT_DOUBLEDOT
                let member = self.rprimary_del()?;
                if range {
                    //r = self.ast_anyty(AstData::Range { from: Box::new(r), to: Box::new(member) });
                    panic!("not yet supported: ..");
                } else {
                    r = self.ast_anyty(AstData::Field { base: Box::new(r), member: Box::new(member) });
                }
                self.next();
            }

            if self.test(TT_LPAREN) {
                let mut params = Vec::new();
                self.rrecord_body(&mut params)?;
                r = self.new_app(Box::new(r), params, AppKind::Normal);
                self.check(TT_RPAREN)?;
            }

            if self.tt == TT_LBRACKET {
                self.next();
                let index = self.rexpr()?;
                r = self.ast_anyty(AstData::Index { base: Box::new(r), index: Box::new(index) });
                self.check(TT_RBRACKET)?;
            }

            /* TODO:
            while (this.tt === Token.LBracket || this.tt === Token.LBrace || this.tt === Token.Backslash) {
                if (this.test(Token.Backslash)) {
                    const e = this.rexpr(astTypeNone); // TODO: Context type from r
                    const l = this.newLambda(e.type);
                    l.expr.push(e);
                    params.push(l);
                } else {
                    const p = this.rprimaryDel(astTypeNone); // TODO: Infer based on matched r
                    this.next(); // TODO: CHECK_NOTERR();
                    params.push(p);
                }
                r = constrainParameter(r, params.length - 1, params[params.length - 1].type);
            }
            */

            if self.tt != TT_DOT && self.tt != TT_LBRACKET && self.tt != TT_LPAREN {
                break;
            }
        }

        Ok(r)
    }

    pub fn rsimple_expr(&mut self) -> ParseResult<Ast> {
        if self.tt == TT_OPIDENT {
            let ident = std::mem::replace(&mut self.token_ident, Ident::new());
            let op = self.ast_anyty(AstData::Ident { s: ident });
            self.next();
            let rhs = self.rsimple_expr()?;
            Ok(self.new_app(Box::new(op), vec![rhs], AppKind::Unary))
        } else if self.test(TT_IF) {
            let cond = self.rexpr()?;
            let body = self.rlambda_block_del()?;
            self.next();
            let else_body;
            if self.test(TT_ELSE) {
                if self.tt == TT_IF {
                    else_body = Some(Box::new(self.rsimple_expr()?));
                } else {
                    let else_lambda = self.rlambda_block_del()?;
                    self.next();
                    else_body = Some(Box::new(
                        self.ast_anyty(AstData::Block { expr: else_lambda.expr })));
                }
            } else {
                else_body = None;
            }
            Ok(self.ast_anyty(AstData::If { cond: Box::new(cond), body, else_body }))
        } else if self.test(TT_WHILE) {
            let cond = self.rexpr()?;
            let body = self.rlambda_block_del()?;
            self.next();
            
            Ok(self.ast_anyty(AstData::While { cond: Box::new(cond), body }))
        } else if self.test(TT_FOR) {
            //let mut locals = vec![];
            //let pat = self.rpattern(&mut locals, AstType::Any)?;
            let name = self.check_ident()?;
            let local_index = self.new_local(
                        name.clone(),
                        AstType::Any,
                        true); // TODO: Immutable let?

            self.check(TT_IN)?;

            let from = self.rexpr()?;
            //self.next();
            self.check(TT_DOUBLEDOT)?;
            //self.check(TT_DOT)?;
            //self.check(TT_DOT)?;
            let to = self.rexpr()?;
            //self.next();
            //r = self.ast_anyty(AstData::Range { from: Box::new(r), to: Box::new(member) });
            //let iter = self.rexpr()?;
            let body = self.rlambda_block_del()?;
            self.next();
            Ok(self.ast_anyty(AstData::For {
                name: name,
                pat: local_index,
                iter: (Box::new(from), Box::new(to)),
                body
            }))
        } else if self.test(TT_BREAK) {
            Ok(self.ast_anyty(AstData::Break))
        } else if self.test(TT_LOOP) {
            let body = self.rlambda_block_del()?;
            self.next();
            
            Ok(self.ast_anyty(AstData::Loop { body }))
        } else if self.test(TT_RETURN) {
            let value;
            if self.tt == TT_RBRACE || self.tt == TT_SEMICOLON {
                value = None;
            } else {
                value = Some(Box::new(self.rexpr()?));
            }
            
            Ok(self.ast_anyty(AstData::Return { value }))
        } else {
            let r = self.rprimary_del()?;
            self.next();

            if self.tt == TT_DOT || self.tt == TT_LPAREN || self.tt == TT_LBRACKET || self.tt == TT_LBRACE || self.tt == TT_BACKSLASH {
                self.rsimple_expr_tail(r)
            } else {
                Ok(r)
            }
        }
    }

    pub fn rexpr_rest(&mut self, mut lhs: Ast, min_prec: u8) -> ParseResult<Ast> {
        while self.tt == TT_OPIDENT {
            let prec = self.token_prec;
            if prec < min_prec { break }
            let op_ident = std::mem::replace(&mut self.token_ident, Ident::new());
            let op = self.ast_anyty(AstData::Ident { s: op_ident });
            self.next();

            /* TODO?
            const isThunk = this.test(Token.Backslash);

            localOp = constrainParameterCount(localOp, 2) as AstLocal;
            try {
                localOp = constrainParameter(localOp, 0, lhs.type) as AstLocal;
            } catch (e) {
                this.error(e.toString());
            }
            */

            let mut rhs = self.rsimple_expr()?;
            while self.tt == TT_OPIDENT {
                let prec2 = self.token_prec;
                // TODO: If current op is right-assoc, use prec2 < prec instead
                if (self.is_rtl && prec2 < prec) || (!self.is_rtl && prec2 <= prec) { break }
                rhs = self.rexpr_rest(rhs, prec2)?;
            }

            /*
            if (isThunk) {
                const l = this.newLambda(rhs.type);
                l.expr.push(rhs);
                rhs = l;
            }
            */

            lhs = self.new_app(Box::new(op), vec![lhs, rhs], AppKind::Binary);
        }

        Ok(lhs)
    }

    pub fn rexpr(&mut self) -> ParseResult<Ast> {
        // TODO: as operator?

        let lhs = self.rsimple_expr()?;
        let r = Ok(self.rexpr_rest(lhs, 0)?);

        r
    }

    pub fn rlambda_module(mut self) -> ModuleResult<Module> {
        let mut lambda = AstLambda {
            params: Vec::new(),
            param_locals: Vec::new(),
            expr: Vec::new(),
            return_type: AstType::Any
        };

        match self.rlambda(&mut lambda, TT_EOF) {

            Ok(_) => {
                Ok(Module {
                    lambda,
                    src: self.src,
                    path: self.path,
                    uses: self.uses,
                    locals: self.locals,
                    local_types: self.local_types,
                    exports: self.exports,
                    exports_rev: self.exports_rev,
                    language: self.language
                })
            }
            Err(err) => {
                Err((self.src, self.path, err))
            }
        }
    }

    pub fn ast_anyty(&self, data: AstData) -> Ast {
        Ast {
            loc: self.span(),
            attr: Attr::None,
            ty: AstType::Any,
            data
        }
    }

    pub fn new_local(&mut self, name: Ident, ty: AstType, is_mut: bool) -> u32 {
        let index = self.locals.len();
        self.locals.push(LocalDef {
            ty, name, is_mut,
            const_value: None,
        });
        index as u32
    }

    pub fn new_local_type(&mut self, name: Ident, ty: AstType) -> u32 {
        let index = self.local_types.len();
        self.local_types.push(TypeDef {
            name, ty
        });
        index as u32
    }

    pub fn ruse_path(&mut self) -> ParseResult<String> {
        let path;
        if self.tt == TT_CONSTSTR {
            path = std::mem::replace(&mut self.token_ident, String::new());
            self.next();
        } else {
            path = self.check_ident()?;
        }
        
        Ok(path)
    }

    pub fn rlambda(&mut self, lambda: &mut AstLambda, end_token: Tt) -> ParseResult<()> {
        'itemloop: loop {
            let mut attr = Attr::None;

            if self.test(TT_AT) {
                let attr_name = self.check_ident()?;
                match &attr_name[..] {
                    "language" => {
                        self.check(TT_LPAREN)?;
                        let lang_name = self.check_ident()?;
                        match &lang_name[..] {
                            "glsl" => self.language = Language::Glsl,
                            _ => panic!("unknown language")
                        }
                        self.check(TT_RPAREN)?;
                        self.test(TT_SEMICOLON);
                        continue 'itemloop;
                    }

                    "attribute" => {
                        attr = Attr::Attribute;
                    }
                    "varying" => {
                        attr = Attr::Varying;
                    }
                    "uniform" => {
                        attr = Attr::Uniform;
                    }
                    "binary" => {
                        attr = Attr::Binary;
                    }
                    "inline" => {
                        attr = Attr::Inline;
                    }
                    "debug" => {
                        attr = Attr::Debug;
                    }
                    _ => panic!("unknown attribute")
                }
                
            }

            let is_pub = self.test(TT_PUB);

            if self.test(TT_FUN) {
                let name = self.check_ident()?;
                let mut params = Vec::new();
                let mut locals = Vec::new();

                if self.test(TT_LPAREN) {
                    self.rparameters(&mut params, &mut locals)?;
                    /*
                    while self.tt != TT_RPAREN {
                        let param_name = self.check_ident()?;
                        let param_ty;
                        if self.test(TT_COLON) {
                            param_ty = self.rtype()?;
                        } else {
                            param_ty = AstType::Any;
                        }

                        let local_index = self.new_local(param_name.clone(), param_ty.clone());
                        params.push(ParamDef { name: param_name, ty: param_ty, local_index });
                        if !self.test_comma_or_semi() {
                            break;
                        }
                    }*/

                    self.check(TT_RPAREN)?;
                }

                let ty;
                if self.test(TT_COLON) {
                    ty = self.rtype()?;
                } else {
                    ty = AstType::None;
                }

                self.check(TT_LBRACE)?;
                let mut sublambda = self.new_lambda(params, locals, ty);
                self.rlambda(&mut sublambda, TT_RBRACE)?;
                //self.check(TT_RBRACE)?;
                self.next();

                // TODO: Set type correctly
                let local_index = self.new_local(
                    name.clone(),
                    AstType::None,
                    false);

                if is_pub {
                    self.exports.push(local_index);
                    self.exports_rev.insert(name.clone(), local_index);
                }

                lambda.expr.push(self.ast_anyty(
                    AstData::FnLocal {
                        name,
                        lambda: sublambda,
                        local_index,
                        exported: is_pub
                    }));
                // TODO: self.skip();
            } else if self.tt == TT_TYPE {
                self.next();
                let name = self.check_ident()?;
                self.check(TT_EQUAL)?;
                let ty = self.rtype()?;

                let index = self.new_local_type(name, ty);
                lambda.expr.push(Ast {
                    loc: self.span(),
                    attr: Attr::None,
                    ty: AstType::None,
                    data: AstData::TypeDef { index }
                });
                
            } else if self.tt == TT_LET || self.tt == TT_VAR {
                let is_mut = self.tt == TT_VAR;

                self.next();

                loop {

                    let name = self.check_ident()?;
                    let ty;
                    if self.test(TT_COLON) {
                        ty = self.rtype()?;
                    } else {
                        // TODO: Decide whether to use None or Any for undeclared types
                        ty = AstType::Any;
                    }

                    let init;
                    if self.test(TT_EQUAL) {
                        init = Some(Box::new(self.rexpr()?));
                        // TODO: Unify types? Do in resolve
                    } else {
                        init = None;
                    }

                    let local_index = self.new_local(
                        name.clone(),
                        ty.clone(),
                        is_mut); // TODO: ty will not be needed below later on

                    if is_pub {
                        self.exports.push(local_index);
                        self.exports_rev.insert(name.clone(), local_index);
                    }

                    lambda.expr.push(Ast {
                        loc: self.span(),
                        attr: Attr::None,
                        ty: AstType::None,
                        data: AstData::LetLocal {
                            name, ty, init, local_index, attr
                        }
                    });

                    if !self.test(TT_COMMA) { // TODO: Allow comma at the end?
                        break;
                    }
                }
            } else if self.test(TT_USE) {
                let path = self.ruse_path()?;
                let name;
                if self.test(TT_AS) {
                    name = std::mem::replace(&mut self.token_ident, String::new());
                    self.next();
                } else {
                    name = path.clone();
                }

                let rel_index = self.uses.len() as u32;
                // TODO: Validate attribute? Do in resolve probably
                self.uses.push(Use(attr, path));
                lambda.expr.push(Ast {
                    loc: self.span(),
                    attr: Attr::None,
                    ty: AstType::None,
                    data: AstData::Use { name, rel_index }
                });
            } else if self.tt != TT_EOF && self.tt != end_token {
                if is_pub {
                    panic!("spurious 'pub'");
                }

                let mut expr = self.rexpr()?;
                expr.attr = attr;
                lambda.expr.push(expr);
            }

            if !self.test(TT_SEMICOLON) {
                break;
            }
        }

        self.peek_check(end_token)?;

        Ok(())
    }
}
