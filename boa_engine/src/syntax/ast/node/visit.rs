use crate::syntax::ast::node::declaration::class_decl::ClassElement;
use crate::syntax::ast::node::declaration::{
    BindingPatternTypeArray, BindingPatternTypeObject, DeclarationPatternArray,
    DeclarationPatternObject,
};
use crate::syntax::ast::node::iteration::IterableLoopInitializer;
use crate::syntax::ast::node::object::{MethodDefinition, PropertyDefinition, PropertyName};
use crate::syntax::ast::node::operator::assign::AssignTarget;
use crate::syntax::ast::node::template::TemplateElement;
use crate::syntax::ast::node::{
    ArrayDecl, ArrowFunctionDecl, Assign, AsyncFunctionDecl, AsyncFunctionExpr, AsyncGeneratorDecl,
    AsyncGeneratorExpr, AwaitExpr, BinOp, Block, Break, Call, Case, Catch, Class, ConditionalOp,
    Continue, Declaration, DeclarationList, DeclarationPattern, DoWhileLoop, Finally, ForInLoop,
    ForLoop, ForOfLoop, FormalParameter, FormalParameterList, FormalParameterListFlags,
    FunctionDecl, FunctionExpr, GeneratorDecl, GeneratorExpr, GetConstField, GetField,
    GetPrivateField, GetSuperField, Identifier, If, New, Object, Return, Spread, StatementList,
    SuperCall, Switch, TaggedTemplate, TemplateLit, Throw, Try, UnaryOp, WhileLoop, Yield,
};
use crate::syntax::ast::{op, Const, Node};
use boa_interner::Sym;

pub trait Visitor<'ast> {
    fn visit_node(&mut self, n: &'ast Node) {
        match n {
            Node::ArrayDecl(n) => self.visit_array_decl(n),
            Node::ArrowFunctionDecl(n) => self.visit_arrow_function_decl(n),
            Node::Assign(n) => self.visit_assign(n),
            Node::AsyncFunctionDecl(n) => self.visit_async_function_decl(n),
            Node::AsyncFunctionExpr(n) => self.visit_async_function_expr(n),
            Node::AsyncGeneratorExpr(n) => self.visit_async_generator_expr(n),
            Node::AsyncGeneratorDecl(n) => self.visit_async_generator_decl(n),
            Node::AwaitExpr(n) => self.visit_await_expr(n),
            Node::BinOp(n) => self.visit_bin_op(n),
            Node::Block(n) => self.visit_block(n),
            Node::Break(n) => self.visit_break(n),
            Node::Call(n) => self.visit_call(n),
            Node::ConditionalOp(n) => self.visit_conditional_op(n),
            Node::Const(n) => self.visit_const(n),
            Node::ConstDeclList(n) | Node::LetDeclList(n) | Node::VarDeclList(n) => {
                self.visit_declaration_list(n);
            }
            Node::Continue(n) => self.visit_continue(n),
            Node::DoWhileLoop(n) => self.visit_do_while_loop(n),
            Node::FunctionDecl(n) => self.visit_function_decl(n),
            Node::FunctionExpr(n) => self.visit_function_expr(n),
            Node::GetConstField(n) => self.visit_get_const_field(n),
            Node::GetPrivateField(n) => self.visit_get_private_field(n),
            Node::GetField(n) => self.visit_get_field(n),
            Node::GetSuperField(n) => self.visit_get_super_field(n),
            Node::ForLoop(n) => self.visit_for_loop(n),
            Node::ForInLoop(n) => self.visit_for_in_loop(n),
            Node::ForOfLoop(n) => self.visit_for_of_loop(n),
            Node::If(n) => self.visit_if(n),
            Node::Identifier(n) => self.visit_identifier(n),
            Node::New(n) => self.visit_new(n),
            Node::Object(n) => self.visit_object(n),
            Node::Return(n) => self.visit_return(n),
            Node::Switch(n) => self.visit_switch(n),
            Node::Spread(n) => self.visit_spread(n),
            Node::TaggedTemplate(n) => self.visit_tagged_template(n),
            Node::TemplateLit(n) => self.visit_template_lit(n),
            Node::Throw(n) => self.visit_throw(n),
            Node::Try(n) => self.visit_try(n),
            Node::UnaryOp(n) => self.visit_unary_op(n),
            Node::WhileLoop(n) => self.visit_while_loop(n),
            Node::Yield(n) => self.visit_yield(n),
            Node::GeneratorDecl(n) => self.visit_generator_decl(n),
            Node::GeneratorExpr(n) => self.visit_generator_expr(n),
            Node::ClassDecl(n) => self.visit_class_decl(n),
            Node::ClassExpr(n) => self.visit_class_expr(n),
            Node::SuperCall(n) => self.visit_super_call(n),
            Node::Empty | Node::This => { /* do nothing */ }
        }
    }

    fn visit_array_decl(&mut self, n: &'ast ArrayDecl) {
        for inner in n.arr.iter() {
            self.visit_node(inner);
        }
    }

    fn visit_arrow_function_decl(&mut self, n: &'ast ArrowFunctionDecl) {
        if let Some(name) = &n.name {
            self.visit_sym(name);
        }
        self.visit_formal_parameter_list(&n.params);
        self.visit_statement_list(&n.body);
    }

    fn visit_assign(&mut self, n: &'ast Assign) {
        self.visit_assign_target(&n.lhs);
        self.visit_node(&n.rhs);
    }

    fn visit_async_function_expr(&mut self, n: &'ast AsyncFunctionExpr) {
        if let Some(name) = &n.name {
            self.visit_sym(name);
        }
        self.visit_formal_parameter_list(&n.parameters);
        self.visit_statement_list(&n.body);
    }

    fn visit_async_function_decl(&mut self, n: &'ast AsyncFunctionDecl) {
        self.visit_sym(&n.name);
        self.visit_formal_parameter_list(&n.parameters);
        self.visit_statement_list(&n.body);
    }

    fn visit_async_generator_expr(&mut self, n: &'ast AsyncGeneratorExpr) {
        if let Some(name) = &n.name {
            self.visit_sym(name);
        }
        self.visit_formal_parameter_list(&n.parameters);
        self.visit_statement_list(&n.body);
    }

    fn visit_async_generator_decl(&mut self, n: &'ast AsyncGeneratorDecl) {
        self.visit_sym(&n.name);
        self.visit_formal_parameter_list(&n.parameters);
        self.visit_statement_list(&n.body);
    }

    fn visit_await_expr(&mut self, n: &'ast AwaitExpr) {
        self.visit_node(&n.expr);
    }

    fn visit_bin_op(&mut self, n: &'ast BinOp) {
        self.visit_raw_binop(&n.op);
        self.visit_node(&n.lhs);
        self.visit_node(&n.rhs);
    }

    fn visit_block(&mut self, n: &'ast Block) {
        self.visit_statement_list(&n.statements);
    }

    fn visit_break(&mut self, n: &'ast Break) {
        if let Some(name) = &n.label {
            self.visit_sym(name);
        }
    }

    fn visit_call(&mut self, n: &'ast Call) {
        self.visit_node(&n.expr);
        for inner in n.args.iter() {
            self.visit_node(inner);
        }
    }

    fn visit_conditional_op(&mut self, n: &'ast ConditionalOp) {
        self.visit_node(&n.condition);
        self.visit_node(&n.if_true);
        self.visit_node(&n.if_false);
    }

    fn visit_const(&mut self, n: &'ast Const) {
        if let Const::String(s) = n {
            self.visit_sym(s);
        }
    }

    fn visit_continue(&mut self, n: &'ast Continue) {
        if let Some(s) = &n.label {
            self.visit_sym(s);
        }
    }

    fn visit_do_while_loop(&mut self, n: &'ast DoWhileLoop) {
        self.visit_node(&n.body);
        self.visit_node(&n.cond);
        if let Some(name) = &n.label {
            self.visit_sym(name);
        }
    }

    fn visit_function_decl(&mut self, n: &'ast FunctionDecl) {
        self.visit_sym(&n.name);
        self.visit_formal_parameter_list(&n.parameters);
        self.visit_statement_list(&n.body);
    }

    fn visit_function_expr(&mut self, n: &'ast FunctionExpr) {
        if let Some(name) = &n.name {
            self.visit_sym(name);
        }
        self.visit_formal_parameter_list(&n.parameters);
        self.visit_statement_list(&n.body);
    }

    fn visit_get_const_field(&mut self, n: &'ast GetConstField) {
        self.visit_node(&n.obj);
        self.visit_sym(&n.field);
    }

    fn visit_get_private_field(&mut self, n: &'ast GetPrivateField) {
        self.visit_node(&n.obj);
        self.visit_sym(&n.field);
    }

    fn visit_get_field(&mut self, n: &'ast GetField) {
        self.visit_node(&n.obj);
        self.visit_node(&n.field);
    }

    fn visit_get_super_field(&mut self, n: &'ast GetSuperField) {
        match n {
            GetSuperField::Const(sym) => self.visit_sym(sym),
            GetSuperField::Expr(n) => self.visit_node(n),
        }
    }

    fn visit_for_loop(&mut self, n: &'ast ForLoop) {
        if let Some(init) = &n.inner.init {
            self.visit_node(init);
        }
        if let Some(condition) = &n.inner.condition {
            self.visit_node(condition);
        }
        if let Some(final_expr) = &n.inner.final_expr {
            self.visit_node(final_expr);
        }
        self.visit_node(&n.inner.body);
        if let Some(name) = &n.label {
            self.visit_sym(name);
        }
    }

    fn visit_for_in_loop(&mut self, n: &'ast ForInLoop) {
        self.visit_iterable_loop_initializer(&n.init);
        self.visit_node(&n.expr);
        self.visit_node(&n.body);
        if let Some(name) = &n.label {
            self.visit_sym(name);
        }
    }

    fn visit_for_of_loop(&mut self, n: &'ast ForOfLoop) {
        self.visit_iterable_loop_initializer(&n.init);
        self.visit_node(&n.iterable);
        self.visit_node(&n.body);
        if let Some(name) = &n.label {
            self.visit_sym(name);
        }
    }

    fn visit_if(&mut self, n: &'ast If) {
        self.visit_node(&n.cond);
        self.visit_node(&n.body);
        if let Some(else_node) = &n.else_node {
            self.visit_node(else_node);
        }
    }

    fn visit_identifier(&mut self, n: &'ast Identifier) {
        self.visit_sym(&n.ident);
    }

    fn visit_new(&mut self, n: &'ast New) {
        self.visit_call(&n.call);
    }

    fn visit_object(&mut self, n: &'ast Object) {
        for pd in n.properties.iter() {
            self.visit_property_definition(pd);
        }
    }

    fn visit_return(&mut self, n: &'ast Return) {
        if let Some(expr) = &n.expr {
            self.visit_node(expr);
        }
        if let Some(name) = &n.label {
            self.visit_sym(name);
        }
    }

    fn visit_switch(&mut self, n: &'ast Switch) {
        self.visit_node(&n.val);
        for case in n.cases.iter() {
            self.visit_case(case);
        }
        if let Some(default) = &n.default {
            self.visit_statement_list(default);
        }
    }

    fn visit_spread(&mut self, n: &'ast Spread) {
        self.visit_node(&n.val);
    }

    fn visit_tagged_template(&mut self, n: &'ast TaggedTemplate) {
        self.visit_node(&n.tag);
        for raw in n.raws.iter() {
            self.visit_sym(raw);
        }
        for cooked in n.cookeds.iter().flatten() {
            self.visit_sym(cooked);
        }
        for expr in n.exprs.iter() {
            self.visit_node(expr);
        }
    }

    fn visit_template_lit(&mut self, n: &'ast TemplateLit) {
        for te in n.elements.iter() {
            self.visit_template_element(te);
        }
    }

    fn visit_throw(&mut self, n: &'ast Throw) {
        self.visit_node(&n.expr);
    }

    fn visit_try(&mut self, n: &'ast Try) {
        self.visit_block(&n.block);
        if let Some(catch) = &n.catch {
            self.visit_catch(catch);
        }
        if let Some(finally) = &n.finally {
            self.visit_finally(finally);
        }
    }

    fn visit_unary_op(&mut self, n: &'ast UnaryOp) {
        self.visit_raw_unary_op(&n.op);
        self.visit_node(&n.target);
    }

    fn visit_declaration_list(&mut self, n: &'ast DeclarationList) {
        match n {
            DeclarationList::Const(decls)
            | DeclarationList::Let(decls)
            | DeclarationList::Var(decls) => {
                for decl in decls.iter() {
                    self.visit_declaration(decl);
                }
            }
        }
    }

    fn visit_while_loop(&mut self, n: &'ast WhileLoop) {
        self.visit_node(&n.cond);
        self.visit_node(&n.body);
        if let Some(name) = &n.label {
            self.visit_sym(name);
        }
    }

    fn visit_yield(&mut self, n: &'ast Yield) {
        if let Some(expr) = &n.expr {
            self.visit_node(expr);
        }
    }

    fn visit_generator_decl(&mut self, n: &'ast GeneratorDecl) {
        self.visit_sym(&n.name);
        self.visit_formal_parameter_list(&n.parameters);
        self.visit_statement_list(&n.body);
    }

    fn visit_generator_expr(&mut self, n: &'ast GeneratorExpr) {
        if let Some(name) = &n.name {
            self.visit_sym(name);
        }
        self.visit_formal_parameter_list(&n.parameters);
        self.visit_statement_list(&n.body);
    }

    fn visit_class_decl(&mut self, n: &'ast Class) {
        self.visit_class(n);
    }

    fn visit_class_expr(&mut self, n: &'ast Class) {
        self.visit_class(n);
    }

    fn visit_class(&mut self, n: &'ast Class) {
        self.visit_sym(&n.name);
        if let Some(super_ref) = &n.super_ref {
            self.visit_node(super_ref);
        }
        if let Some(constructor) = &n.constructor {
            self.visit_function_expr(constructor);
        }
        for elem in n.elements.iter() {
            self.visit_class_element(elem);
        }
    }

    fn visit_class_element(&mut self, n: &'ast ClassElement) {
        match n {
            ClassElement::MethodDefinition(pn, md)
            | ClassElement::StaticMethodDefinition(pn, md) => {
                self.visit_property_name(pn);
                self.visit_method_definition(md);
            }
            ClassElement::FieldDefinition(pn, fd) | ClassElement::StaticFieldDefinition(pn, fd) => {
                self.visit_property_name(pn);
                if let Some(n) = fd {
                    self.visit_node(n);
                }
            }
            ClassElement::PrivateMethodDefinition(s, md)
            | ClassElement::PrivateStaticMethodDefinition(s, md) => {
                self.visit_sym(s);
                self.visit_method_definition(md);
            }
            ClassElement::PrivateFieldDefinition(s, fd)
            | ClassElement::PrivateStaticFieldDefinition(s, fd) => {
                self.visit_sym(s);
                if let Some(n) = fd {
                    self.visit_node(n);
                }
            }
            ClassElement::StaticBlock(sl) => {
                self.visit_statement_list(sl);
            }
        }
    }

    fn visit_super_call(&mut self, n: &'ast SuperCall) {
        for arg in n.args.iter() {
            self.visit_node(arg);
        }
    }

    fn visit_sym(&mut self, _n: &'ast Sym) {
        /* do nothing */
    }

    fn visit_formal_parameter_list(&mut self, n: &'ast FormalParameterList) {
        for p in n.parameters.iter() {
            self.visit_formal_parameter(p);
        }
        self.visit_formal_parameter_list_flags(&n.flags);
    }

    fn visit_statement_list(&mut self, n: &'ast StatementList) {
        for inner in n.items.iter() {
            self.visit_node(inner);
        }
    }

    fn visit_assign_target(&mut self, n: &'ast AssignTarget) {
        match n {
            AssignTarget::Identifier(ident) => self.visit_identifier(ident),
            AssignTarget::GetPrivateField(gpf) => self.visit_get_private_field(gpf),
            AssignTarget::GetConstField(gcf) => self.visit_get_const_field(gcf),
            AssignTarget::GetField(gf) => self.visit_get_field(gf),
            AssignTarget::DeclarationPattern(dp) => self.visit_declaration_pattern(dp),
        }
    }

    fn visit_raw_binop(&mut self, n: &'ast op::BinOp) {
        match n {
            op::BinOp::Num(op) => self.visit_raw_num_op(op),
            op::BinOp::Bit(op) => self.visit_raw_bit_op(op),
            op::BinOp::Comp(op) => self.visit_raw_comp_op(op),
            op::BinOp::Log(op) => self.visit_raw_log_op(op),
            op::BinOp::Assign(op) => self.visit_raw_assign_op(op),
            op::BinOp::Comma => {}
        }
    }

    fn visit_declaration(&mut self, n: &'ast Declaration) {
        match n {
            Declaration::Identifier { ident, init } => {
                self.visit_identifier(ident);
                if let Some(init) = init {
                    self.visit_node(init);
                }
            }
            Declaration::Pattern(dp) => self.visit_declaration_pattern(dp),
        }
    }

    fn visit_iterable_loop_initializer(&mut self, n: &'ast IterableLoopInitializer) {
        match n {
            IterableLoopInitializer::Identifier(ident) => self.visit_identifier(ident),
            IterableLoopInitializer::Var(decl)
            | IterableLoopInitializer::Let(decl)
            | IterableLoopInitializer::Const(decl) => self.visit_declaration(decl),
            IterableLoopInitializer::DeclarationPattern(dp) => self.visit_declaration_pattern(dp),
        }
    }

    fn visit_property_definition(&mut self, n: &'ast PropertyDefinition) {
        match n {
            PropertyDefinition::IdentifierReference(s) => self.visit_sym(s),
            PropertyDefinition::Property(pn, inner) => {
                self.visit_property_name(pn);
                self.visit_node(inner);
            }
            PropertyDefinition::MethodDefinition(md, pn) => {
                self.visit_method_definition(md);
                self.visit_property_name(pn);
            }
            PropertyDefinition::SpreadObject(inner) => self.visit_node(inner),
        }
    }

    fn visit_case(&mut self, n: &'ast Case) {
        self.visit_node(&n.condition);
        self.visit_statement_list(&n.body);
    }

    fn visit_template_element(&mut self, n: &'ast TemplateElement) {
        match n {
            TemplateElement::String(s) => self.visit_sym(s),
            TemplateElement::Expr(inner) => self.visit_node(inner),
        }
    }

    fn visit_catch(&mut self, n: &'ast Catch) {
        if let Some(parameter) = &n.parameter {
            self.visit_declaration(parameter);
        }
        self.visit_block(&n.block);
    }

    fn visit_finally(&mut self, n: &'ast Finally) {
        self.visit_block(&n.block);
    }

    fn visit_raw_unary_op(&mut self, _n: &'ast op::UnaryOp) {
        /* do nothing */
    }

    fn visit_formal_parameter(&mut self, n: &'ast FormalParameter) {
        self.visit_declaration(&n.declaration);
    }

    fn visit_formal_parameter_list_flags(&mut self, _n: &'ast FormalParameterListFlags) {
        /* do nothing */
    }

    fn visit_declaration_pattern(&mut self, n: &'ast DeclarationPattern) {
        match n {
            DeclarationPattern::Object(o) => self.visit_declaration_pattern_object(o),
            DeclarationPattern::Array(a) => self.visit_declaration_pattern_array(a),
        }
    }

    fn visit_raw_num_op(&mut self, _n: &'ast op::NumOp) {
        /* do nothing */
    }

    fn visit_raw_bit_op(&mut self, _n: &'ast op::BitOp) {
        /* do nothing */
    }

    fn visit_raw_comp_op(&mut self, _n: &'ast op::CompOp) {
        /* do nothing */
    }

    fn visit_raw_log_op(&mut self, _n: &'ast op::LogOp) {
        /* do nothing */
    }

    fn visit_raw_assign_op(&mut self, _n: &'ast op::AssignOp) {
        /* do nothing */
    }

    fn visit_property_name(&mut self, n: &'ast PropertyName) {
        match n {
            PropertyName::Literal(s) => self.visit_sym(s),
            PropertyName::Computed(inner) => self.visit_node(inner),
        }
    }

    fn visit_method_definition(&mut self, n: &'ast MethodDefinition) {
        match n {
            MethodDefinition::Get(fe)
            | MethodDefinition::Set(fe)
            | MethodDefinition::Ordinary(fe) => self.visit_function_expr(fe),
            MethodDefinition::Generator(ge) => self.visit_generator_expr(ge),
            MethodDefinition::AsyncGenerator(age) => self.visit_async_generator_expr(age),
            MethodDefinition::Async(afe) => self.visit_async_function_expr(afe),
        }
    }

    fn visit_declaration_pattern_object(&mut self, n: &'ast DeclarationPatternObject) {
        for binding in &n.bindings {
            self.visit_binding_pattern_type_object(binding);
        }
        if let Some(init) = &n.init {
            self.visit_node(init);
        }
    }

    fn visit_declaration_pattern_array(&mut self, n: &'ast DeclarationPatternArray) {
        for binding in &n.bindings {
            self.visit_binding_pattern_type_array(binding);
        }
        if let Some(init) = &n.init {
            self.visit_node(init);
        }
    }

    fn visit_binding_pattern_type_object(&mut self, n: &'ast BindingPatternTypeObject) {
        match n {
            BindingPatternTypeObject::Empty => {}
            BindingPatternTypeObject::SingleName {
                ident,
                property_name,
                default_init,
            } => {
                self.visit_sym(ident);
                self.visit_property_name(property_name);
                if let Some(init) = default_init {
                    self.visit_node(init);
                }
            }
            BindingPatternTypeObject::RestProperty {
                ident,
                excluded_keys,
            } => {
                self.visit_sym(ident);
                for key in excluded_keys.iter() {
                    self.visit_sym(key);
                }
            }
            BindingPatternTypeObject::RestGetConstField {
                get_const_field,
                excluded_keys,
            } => {
                self.visit_get_const_field(get_const_field);
                for key in excluded_keys.iter() {
                    self.visit_sym(key);
                }
            }
            BindingPatternTypeObject::BindingPattern {
                ident,
                pattern,
                default_init,
            } => {
                self.visit_property_name(ident);
                self.visit_declaration_pattern(pattern);
                if let Some(init) = default_init {
                    self.visit_node(init);
                }
            }
        }
    }

    fn visit_binding_pattern_type_array(&mut self, n: &'ast BindingPatternTypeArray) {
        match n {
            BindingPatternTypeArray::SingleName {
                ident,
                default_init,
            } => {
                self.visit_sym(ident);
                if let Some(init) = default_init {
                    self.visit_node(init);
                }
            }
            BindingPatternTypeArray::GetField { get_field }
            | BindingPatternTypeArray::GetFieldRest { get_field } => {
                self.visit_get_field(get_field);
            }
            BindingPatternTypeArray::GetConstField { get_const_field }
            | BindingPatternTypeArray::GetConstFieldRest { get_const_field } => {
                self.visit_get_const_field(get_const_field);
            }
            BindingPatternTypeArray::BindingPattern { pattern }
            | BindingPatternTypeArray::BindingPatternRest { pattern } => {
                self.visit_declaration_pattern(pattern);
            }
            BindingPatternTypeArray::SingleNameRest { ident } => self.visit_sym(ident),
            BindingPatternTypeArray::Empty | BindingPatternTypeArray::Elision => {}
        }
    }

    fn visit_node_mut(&mut self, n: &'ast mut Node) {
        match n {
            Node::ArrayDecl(n) => self.visit_array_decl_mut(n),
            Node::ArrowFunctionDecl(n) => self.visit_arrow_function_decl_mut(n),
            Node::Assign(n) => self.visit_assign_mut(n),
            Node::AsyncFunctionDecl(n) => self.visit_async_function_decl_mut(n),
            Node::AsyncFunctionExpr(n) => self.visit_async_function_expr_mut(n),
            Node::AsyncGeneratorExpr(n) => self.visit_async_generator_expr_mut(n),
            Node::AsyncGeneratorDecl(n) => self.visit_async_generator_decl_mut(n),
            Node::AwaitExpr(n) => self.visit_await_expr_mut(n),
            Node::BinOp(n) => self.visit_bin_op_mut(n),
            Node::Block(n) => self.visit_block_mut(n),
            Node::Break(n) => self.visit_break_mut(n),
            Node::Call(n) => self.visit_call_mut(n),
            Node::ConditionalOp(n) => self.visit_conditional_op_mut(n),
            Node::Const(n) => self.visit_const_mut(n),
            Node::ConstDeclList(n) | Node::LetDeclList(n) | Node::VarDeclList(n) => {
                self.visit_declaration_list_mut(n);
            }
            Node::Continue(n) => self.visit_continue_mut(n),
            Node::DoWhileLoop(n) => self.visit_do_while_loop_mut(n),
            Node::FunctionDecl(n) => self.visit_function_decl_mut(n),
            Node::FunctionExpr(n) => self.visit_function_expr_mut(n),
            Node::GetConstField(n) => self.visit_get_const_field_mut(n),
            Node::GetPrivateField(n) => self.visit_get_private_field_mut(n),
            Node::GetField(n) => self.visit_get_field_mut(n),
            Node::GetSuperField(n) => self.visit_get_super_field_mut(n),
            Node::ForLoop(n) => self.visit_for_loop_mut(n),
            Node::ForInLoop(n) => self.visit_for_in_loop_mut(n),
            Node::ForOfLoop(n) => self.visit_for_of_loop_mut(n),
            Node::If(n) => self.visit_if_mut(n),
            Node::Identifier(n) => self.visit_identifier_mut(n),
            Node::New(n) => self.visit_new_mut(n),
            Node::Object(n) => self.visit_object_mut(n),
            Node::Return(n) => self.visit_return_mut(n),
            Node::Switch(n) => self.visit_switch_mut(n),
            Node::Spread(n) => self.visit_spread_mut(n),
            Node::TaggedTemplate(n) => self.visit_tagged_template_mut(n),
            Node::TemplateLit(n) => self.visit_template_lit_mut(n),
            Node::Throw(n) => self.visit_throw_mut(n),
            Node::Try(n) => self.visit_try_mut(n),
            Node::UnaryOp(n) => self.visit_unary_op_mut(n),
            Node::WhileLoop(n) => self.visit_while_loop_mut(n),
            Node::Yield(n) => self.visit_yield_mut(n),
            Node::GeneratorDecl(n) => self.visit_generator_decl_mut(n),
            Node::GeneratorExpr(n) => self.visit_generator_expr_mut(n),
            Node::ClassDecl(n) => self.visit_class_decl_mut(n),
            Node::ClassExpr(n) => self.visit_class_expr_mut(n),
            Node::SuperCall(n) => self.visit_super_call_mut(n),
            Node::Empty | Node::This => { /* do nothing */ }
        }
    }

    fn visit_array_decl_mut(&mut self, n: &'ast mut ArrayDecl) {
        for inner in n.arr.iter_mut() {
            self.visit_node_mut(inner);
        }
    }

    fn visit_arrow_function_decl_mut(&mut self, n: &'ast mut ArrowFunctionDecl) {
        if let Some(name) = &mut n.name {
            self.visit_sym_mut(name);
        }
        self.visit_formal_parameter_list_mut(&mut n.params);
        self.visit_statement_list_mut(&mut n.body);
    }

    fn visit_assign_mut(&mut self, n: &'ast mut Assign) {
        self.visit_assign_target_mut(&mut n.lhs);
        self.visit_node_mut(&mut n.rhs);
    }

    fn visit_async_function_expr_mut(&mut self, n: &'ast mut AsyncFunctionExpr) {
        if let Some(name) = &mut n.name {
            self.visit_sym_mut(name);
        }
        self.visit_formal_parameter_list_mut(&mut n.parameters);
        self.visit_statement_list_mut(&mut n.body);
    }

    fn visit_async_function_decl_mut(&mut self, n: &'ast mut AsyncFunctionDecl) {
        self.visit_sym_mut(&mut n.name);
        self.visit_formal_parameter_list_mut(&mut n.parameters);
        self.visit_statement_list_mut(&mut n.body);
    }

    fn visit_async_generator_expr_mut(&mut self, n: &'ast mut AsyncGeneratorExpr) {
        if let Some(name) = &mut n.name {
            self.visit_sym_mut(name);
        }
        self.visit_formal_parameter_list_mut(&mut n.parameters);
        self.visit_statement_list_mut(&mut n.body);
    }

    fn visit_async_generator_decl_mut(&mut self, n: &'ast mut AsyncGeneratorDecl) {
        self.visit_sym_mut(&mut n.name);
        self.visit_formal_parameter_list_mut(&mut n.parameters);
        self.visit_statement_list_mut(&mut n.body);
    }

    fn visit_await_expr_mut(&mut self, n: &'ast mut AwaitExpr) {
        self.visit_node_mut(&mut n.expr);
    }

    fn visit_bin_op_mut(&mut self, n: &'ast mut BinOp) {
        self.visit_raw_binop_mut(&mut n.op);
        self.visit_node_mut(&mut n.lhs);
        self.visit_node_mut(&mut n.rhs);
    }

    fn visit_block_mut(&mut self, n: &'ast mut Block) {
        self.visit_statement_list_mut(&mut n.statements);
    }

    fn visit_break_mut(&mut self, n: &'ast mut Break) {
        if let Some(name) = &mut n.label {
            self.visit_sym_mut(name);
        }
    }

    fn visit_call_mut(&mut self, n: &'ast mut Call) {
        self.visit_node_mut(&mut n.expr);
        for inner in n.args.iter_mut() {
            self.visit_node_mut(inner);
        }
    }

    fn visit_conditional_op_mut(&mut self, n: &'ast mut ConditionalOp) {
        self.visit_node_mut(&mut n.condition);
        self.visit_node_mut(&mut n.if_true);
        self.visit_node_mut(&mut n.if_false);
    }

    fn visit_const_mut(&mut self, n: &'ast mut Const) {
        if let Const::String(s) = n {
            self.visit_sym_mut(s);
        }
    }

    fn visit_continue_mut(&mut self, n: &'ast mut Continue) {
        if let Some(s) = &mut n.label {
            self.visit_sym_mut(s);
        }
    }

    fn visit_do_while_loop_mut(&mut self, n: &'ast mut DoWhileLoop) {
        self.visit_node_mut(&mut n.body);
        self.visit_node_mut(&mut n.cond);
        if let Some(name) = &mut n.label {
            self.visit_sym_mut(name);
        }
    }

    fn visit_function_decl_mut(&mut self, n: &'ast mut FunctionDecl) {
        self.visit_sym_mut(&mut n.name);
        self.visit_formal_parameter_list_mut(&mut n.parameters);
        self.visit_statement_list_mut(&mut n.body);
    }

    fn visit_function_expr_mut(&mut self, n: &'ast mut FunctionExpr) {
        if let Some(name) = &mut n.name {
            self.visit_sym_mut(name);
        }
        self.visit_formal_parameter_list_mut(&mut n.parameters);
        self.visit_statement_list_mut(&mut n.body);
    }

    fn visit_get_const_field_mut(&mut self, n: &'ast mut GetConstField) {
        self.visit_node_mut(&mut n.obj);
        self.visit_sym_mut(&mut n.field);
    }

    fn visit_get_private_field_mut(&mut self, n: &'ast mut GetPrivateField) {
        self.visit_node_mut(&mut n.obj);
        self.visit_sym_mut(&mut n.field);
    }

    fn visit_get_field_mut(&mut self, n: &'ast mut GetField) {
        self.visit_node_mut(&mut n.obj);
        self.visit_node_mut(&mut n.field);
    }

    fn visit_get_super_field_mut(&mut self, n: &'ast mut GetSuperField) {
        match n {
            GetSuperField::Const(sym) => self.visit_sym_mut(sym),
            GetSuperField::Expr(n) => self.visit_node_mut(n.as_mut()),
        }
    }

    fn visit_for_loop_mut(&mut self, n: &'ast mut ForLoop) {
        if let Some(init) = &mut n.inner.init {
            self.visit_node_mut(init);
        }
        if let Some(condition) = &mut n.inner.condition {
            self.visit_node_mut(condition);
        }
        if let Some(final_expr) = &mut n.inner.final_expr {
            self.visit_node_mut(final_expr);
        }
        self.visit_node_mut(&mut n.inner.body);
        if let Some(name) = &mut n.label {
            self.visit_sym_mut(name);
        }
    }

    fn visit_for_in_loop_mut(&mut self, n: &'ast mut ForInLoop) {
        self.visit_iterable_loop_initializer_mut(&mut n.init);
        self.visit_node_mut(&mut n.expr);
        self.visit_node_mut(&mut n.body);
        if let Some(name) = &mut n.label {
            self.visit_sym_mut(name);
        }
    }

    fn visit_for_of_loop_mut(&mut self, n: &'ast mut ForOfLoop) {
        self.visit_iterable_loop_initializer_mut(&mut n.init);
        self.visit_node_mut(&mut n.iterable);
        self.visit_node_mut(&mut n.body);
        if let Some(name) = &mut n.label {
            self.visit_sym_mut(name);
        }
    }

    fn visit_if_mut(&mut self, n: &'ast mut If) {
        self.visit_node_mut(&mut n.cond);
        self.visit_node_mut(&mut n.body);
        if let Some(else_node) = &mut n.else_node {
            self.visit_node_mut(else_node.as_mut());
        }
    }

    fn visit_identifier_mut(&mut self, n: &'ast mut Identifier) {
        self.visit_sym_mut(&mut n.ident);
    }

    fn visit_new_mut(&mut self, n: &'ast mut New) {
        self.visit_call_mut(&mut n.call);
    }

    fn visit_object_mut(&mut self, n: &'ast mut Object) {
        for pd in n.properties.iter_mut() {
            self.visit_property_definition_mut(pd);
        }
    }

    fn visit_return_mut(&mut self, n: &'ast mut Return) {
        if let Some(expr) = &mut n.expr {
            self.visit_node_mut(expr.as_mut());
        }
        if let Some(name) = &mut n.label {
            self.visit_sym_mut(name);
        }
    }

    fn visit_switch_mut(&mut self, n: &'ast mut Switch) {
        self.visit_node_mut(&mut n.val);
        for case in n.cases.iter_mut() {
            self.visit_case_mut(case);
        }
        if let Some(default) = &mut n.default {
            self.visit_statement_list_mut(default);
        }
    }

    fn visit_spread_mut(&mut self, n: &'ast mut Spread) {
        self.visit_node_mut(&mut n.val);
    }

    fn visit_tagged_template_mut(&mut self, n: &'ast mut TaggedTemplate) {
        self.visit_node_mut(&mut n.tag);
        for raw in n.raws.iter_mut() {
            self.visit_sym_mut(raw);
        }
        for cooked in n.cookeds.iter_mut().flatten() {
            self.visit_sym_mut(cooked);
        }
        for expr in n.exprs.iter_mut() {
            self.visit_node_mut(expr);
        }
    }

    fn visit_template_lit_mut(&mut self, n: &'ast mut TemplateLit) {
        for te in n.elements.iter_mut() {
            self.visit_template_element_mut(te);
        }
    }

    fn visit_throw_mut(&mut self, n: &'ast mut Throw) {
        self.visit_node_mut(&mut n.expr);
    }

    fn visit_try_mut(&mut self, n: &'ast mut Try) {
        self.visit_block_mut(&mut n.block);
        if let Some(catch) = &mut n.catch {
            self.visit_catch_mut(catch);
        }
        if let Some(finally) = &mut n.finally {
            self.visit_finally_mut(finally);
        }
    }

    fn visit_unary_op_mut(&mut self, n: &'ast mut UnaryOp) {
        self.visit_raw_unary_op_mut(&mut n.op);
        self.visit_node_mut(&mut n.target);
    }

    fn visit_declaration_list_mut(&mut self, n: &'ast mut DeclarationList) {
        match n {
            DeclarationList::Const(decls)
            | DeclarationList::Let(decls)
            | DeclarationList::Var(decls) => {
                for decl in decls.iter_mut() {
                    self.visit_declaration_mut(decl);
                }
            }
        }
    }

    fn visit_while_loop_mut(&mut self, n: &'ast mut WhileLoop) {
        self.visit_node_mut(&mut n.cond);
        self.visit_node_mut(&mut n.body);
        if let Some(name) = &mut n.label {
            self.visit_sym_mut(name);
        }
    }

    fn visit_yield_mut(&mut self, n: &'ast mut Yield) {
        if let Some(expr) = &mut n.expr {
            self.visit_node_mut(expr.as_mut());
        }
    }

    fn visit_generator_decl_mut(&mut self, n: &'ast mut GeneratorDecl) {
        self.visit_sym_mut(&mut n.name);
        self.visit_formal_parameter_list_mut(&mut n.parameters);
        self.visit_statement_list_mut(&mut n.body);
    }

    fn visit_generator_expr_mut(&mut self, n: &'ast mut GeneratorExpr) {
        if let Some(name) = &mut n.name {
            self.visit_sym_mut(name);
        }
        self.visit_formal_parameter_list_mut(&mut n.parameters);
        self.visit_statement_list_mut(&mut n.body);
    }

    fn visit_class_decl_mut(&mut self, n: &'ast mut Class) {
        self.visit_class_mut(n);
    }

    fn visit_class_expr_mut(&mut self, n: &'ast mut Class) {
        self.visit_class_mut(n);
    }

    fn visit_class_mut(&mut self, n: &'ast mut Class) {
        self.visit_sym_mut(&mut n.name);
        if let Some(super_ref) = n.super_ref.as_deref_mut() {
            self.visit_node_mut(super_ref);
        }
        if let Some(constructor) = &mut n.constructor {
            self.visit_function_expr_mut(constructor);
        }
        for elem in n.elements.iter_mut() {
            self.visit_class_element_mut(elem);
        }
    }

    fn visit_class_element_mut(&mut self, n: &'ast mut ClassElement) {
        match n {
            ClassElement::MethodDefinition(pn, md)
            | ClassElement::StaticMethodDefinition(pn, md) => {
                self.visit_property_name_mut(pn);
                self.visit_method_definition_mut(md);
            }
            ClassElement::FieldDefinition(pn, fd) | ClassElement::StaticFieldDefinition(pn, fd) => {
                self.visit_property_name_mut(pn);
                if let Some(n) = fd {
                    self.visit_node_mut(n);
                }
            }
            ClassElement::PrivateMethodDefinition(s, md)
            | ClassElement::PrivateStaticMethodDefinition(s, md) => {
                self.visit_sym_mut(s);
                self.visit_method_definition_mut(md);
            }
            ClassElement::PrivateFieldDefinition(s, fd)
            | ClassElement::PrivateStaticFieldDefinition(s, fd) => {
                self.visit_sym_mut(s);
                if let Some(n) = fd {
                    self.visit_node_mut(n);
                }
            }
            ClassElement::StaticBlock(sl) => {
                self.visit_statement_list_mut(sl);
            }
        }
    }

    fn visit_super_call_mut(&mut self, n: &'ast mut SuperCall) {
        for arg in n.args.iter_mut() {
            self.visit_node_mut(arg);
        }
    }

    fn visit_sym_mut(&mut self, _n: &'ast mut Sym) {
        /* do nothing */
    }

    fn visit_formal_parameter_list_mut(&mut self, n: &'ast mut FormalParameterList) {
        for p in n.parameters.iter_mut() {
            self.visit_formal_parameter_mut(p);
        }
        self.visit_formal_parameter_list_flags_mut(&mut n.flags);
    }

    fn visit_statement_list_mut(&mut self, n: &'ast mut StatementList) {
        for inner in n.items.iter_mut() {
            self.visit_node_mut(inner);
        }
    }

    fn visit_assign_target_mut(&mut self, n: &'ast mut AssignTarget) {
        match n {
            AssignTarget::Identifier(ident) => self.visit_identifier_mut(ident),
            AssignTarget::GetPrivateField(gpf) => self.visit_get_private_field_mut(gpf),
            AssignTarget::GetConstField(gcf) => self.visit_get_const_field_mut(gcf),
            AssignTarget::GetField(gf) => self.visit_get_field_mut(gf),
            AssignTarget::DeclarationPattern(dp) => self.visit_declaration_pattern_mut(dp),
        }
    }

    fn visit_raw_binop_mut(&mut self, n: &'ast mut op::BinOp) {
        match n {
            op::BinOp::Num(op) => self.visit_raw_num_op_mut(op),
            op::BinOp::Bit(op) => self.visit_raw_bit_op_mut(op),
            op::BinOp::Comp(op) => self.visit_raw_comp_op_mut(op),
            op::BinOp::Log(op) => self.visit_raw_log_op_mut(op),
            op::BinOp::Assign(op) => self.visit_raw_assign_op_mut(op),
            op::BinOp::Comma => {}
        }
    }

    fn visit_declaration_mut(&mut self, n: &'ast mut Declaration) {
        match n {
            Declaration::Identifier { ident, init } => {
                self.visit_identifier_mut(ident);
                if let Some(init) = init {
                    self.visit_node_mut(init);
                }
            }
            Declaration::Pattern(dp) => self.visit_declaration_pattern_mut(dp),
        }
    }

    fn visit_iterable_loop_initializer_mut(&mut self, n: &'ast mut IterableLoopInitializer) {
        match n {
            IterableLoopInitializer::Identifier(ident) => self.visit_identifier_mut(ident),
            IterableLoopInitializer::Var(decl)
            | IterableLoopInitializer::Let(decl)
            | IterableLoopInitializer::Const(decl) => self.visit_declaration_mut(decl),
            IterableLoopInitializer::DeclarationPattern(dp) => {
                self.visit_declaration_pattern_mut(dp);
            }
        }
    }

    fn visit_property_definition_mut(&mut self, n: &'ast mut PropertyDefinition) {
        match n {
            PropertyDefinition::IdentifierReference(s) => self.visit_sym_mut(s),
            PropertyDefinition::Property(pn, inner) => {
                self.visit_property_name_mut(pn);
                self.visit_node_mut(inner);
            }
            PropertyDefinition::MethodDefinition(md, pn) => {
                self.visit_method_definition_mut(md);
                self.visit_property_name_mut(pn);
            }
            PropertyDefinition::SpreadObject(inner) => self.visit_node_mut(inner),
        }
    }

    fn visit_case_mut(&mut self, n: &'ast mut Case) {
        self.visit_node_mut(&mut n.condition);
        self.visit_statement_list_mut(&mut n.body);
    }

    fn visit_template_element_mut(&mut self, n: &'ast mut TemplateElement) {
        match n {
            TemplateElement::String(s) => self.visit_sym_mut(s),
            TemplateElement::Expr(inner) => self.visit_node_mut(inner),
        }
    }

    fn visit_catch_mut(&mut self, n: &'ast mut Catch) {
        if let Some(parameter) = &mut n.parameter {
            self.visit_declaration_mut(parameter.as_mut());
        }
        self.visit_block_mut(&mut n.block);
    }

    fn visit_finally_mut(&mut self, n: &'ast mut Finally) {
        self.visit_block_mut(&mut n.block);
    }

    fn visit_raw_unary_op_mut(&mut self, _n: &'ast mut op::UnaryOp) {
        /* do nothing */
    }

    fn visit_formal_parameter_mut(&mut self, n: &'ast mut FormalParameter) {
        self.visit_declaration_mut(&mut n.declaration);
    }

    fn visit_formal_parameter_list_flags_mut(&mut self, _n: &'ast mut FormalParameterListFlags) {
        /* do nothing */
    }

    fn visit_declaration_pattern_mut(&mut self, n: &'ast mut DeclarationPattern) {
        match n {
            DeclarationPattern::Object(o) => self.visit_declaration_pattern_object_mut(o),
            DeclarationPattern::Array(a) => self.visit_declaration_pattern_array_mut(a),
        }
    }

    fn visit_raw_num_op_mut(&mut self, _n: &'ast mut op::NumOp) {
        /* do nothing */
    }

    fn visit_raw_bit_op_mut(&mut self, _n: &'ast mut op::BitOp) {
        /* do nothing */
    }

    fn visit_raw_comp_op_mut(&mut self, _n: &'ast mut op::CompOp) {
        /* do nothing */
    }

    fn visit_raw_log_op_mut(&mut self, _n: &'ast mut op::LogOp) {
        /* do nothing */
    }

    fn visit_raw_assign_op_mut(&mut self, _n: &'ast mut op::AssignOp) {
        /* do nothing */
    }

    fn visit_property_name_mut(&mut self, n: &'ast mut PropertyName) {
        match n {
            PropertyName::Literal(s) => self.visit_sym_mut(s),
            PropertyName::Computed(inner) => self.visit_node_mut(inner),
        }
    }

    fn visit_method_definition_mut(&mut self, n: &'ast mut MethodDefinition) {
        match n {
            MethodDefinition::Get(fe)
            | MethodDefinition::Set(fe)
            | MethodDefinition::Ordinary(fe) => self.visit_function_expr_mut(fe),
            MethodDefinition::Generator(ge) => self.visit_generator_expr_mut(ge),
            MethodDefinition::AsyncGenerator(age) => self.visit_async_generator_expr_mut(age),
            MethodDefinition::Async(afe) => self.visit_async_function_expr_mut(afe),
        }
    }

    fn visit_declaration_pattern_object_mut(&mut self, n: &'ast mut DeclarationPatternObject) {
        for binding in &mut n.bindings {
            self.visit_binding_pattern_type_object_mut(binding);
        }
        if let Some(init) = &mut n.init {
            self.visit_node_mut(init);
        }
    }

    fn visit_declaration_pattern_array_mut(&mut self, n: &'ast mut DeclarationPatternArray) {
        for binding in &mut n.bindings {
            self.visit_binding_pattern_type_array_mut(binding);
        }
        if let Some(init) = &mut n.init {
            self.visit_node_mut(init);
        }
    }

    fn visit_binding_pattern_type_object_mut(&mut self, n: &'ast mut BindingPatternTypeObject) {
        match n {
            BindingPatternTypeObject::Empty => {}
            BindingPatternTypeObject::SingleName {
                ident,
                property_name,
                default_init,
            } => {
                self.visit_sym_mut(ident);
                self.visit_property_name_mut(property_name);
                if let Some(init) = default_init {
                    self.visit_node_mut(init);
                }
            }
            BindingPatternTypeObject::RestProperty {
                ident,
                excluded_keys,
            } => {
                self.visit_sym_mut(ident);
                for key in excluded_keys.iter_mut() {
                    self.visit_sym_mut(key);
                }
            }
            BindingPatternTypeObject::RestGetConstField {
                get_const_field,
                excluded_keys,
            } => {
                self.visit_get_const_field_mut(get_const_field);
                for key in excluded_keys.iter_mut() {
                    self.visit_sym_mut(key);
                }
            }
            BindingPatternTypeObject::BindingPattern {
                ident,
                pattern,
                default_init,
            } => {
                self.visit_property_name_mut(ident);
                self.visit_declaration_pattern_mut(pattern);
                if let Some(init) = default_init {
                    self.visit_node_mut(init);
                }
            }
        }
    }

    fn visit_binding_pattern_type_array_mut(&mut self, n: &'ast mut BindingPatternTypeArray) {
        match n {
            BindingPatternTypeArray::SingleName {
                ident,
                default_init,
            } => {
                self.visit_sym_mut(ident);
                if let Some(init) = default_init {
                    self.visit_node_mut(init);
                }
            }
            BindingPatternTypeArray::GetField { get_field }
            | BindingPatternTypeArray::GetFieldRest { get_field } => {
                self.visit_get_field_mut(get_field);
            }
            BindingPatternTypeArray::GetConstField { get_const_field }
            | BindingPatternTypeArray::GetConstFieldRest { get_const_field } => {
                self.visit_get_const_field_mut(get_const_field);
            }
            BindingPatternTypeArray::BindingPattern { pattern }
            | BindingPatternTypeArray::BindingPatternRest { pattern } => {
                self.visit_declaration_pattern_mut(pattern);
            }
            BindingPatternTypeArray::SingleNameRest { ident } => self.visit_sym_mut(ident),
            BindingPatternTypeArray::Empty | BindingPatternTypeArray::Elision => {}
        }
    }
}
