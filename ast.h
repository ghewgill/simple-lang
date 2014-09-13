#ifndef AST_H
#define AST_H

#include <string>
#include <vector>

// Compiler
class Emitter;

class AstNode {
public:
    AstNode() {}
    void dump(int depth = 0) const;
    virtual std::string text() const = 0;
    virtual void dumpsubnodes(int depth) const {}
private:
    AstNode(const AstNode &);
    AstNode &operator=(const AstNode &);
};

class Expression: public AstNode {
public:
    virtual void generate(Emitter &emitter) const = 0;
};

class ConstantExpression: public Expression {
public:
    ConstantExpression(int value): value(value) {}

    const int value;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const;
};

class UnaryMinusExpression: public Expression {
public:
    UnaryMinusExpression(const Expression *value): value(value) {}

    const Expression *const value;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const {
        return "UnaryMinusExpression(" + value->text() + ")";
    }
};

class AdditionExpression: public Expression {
public:
    AdditionExpression(const Expression *left, const Expression *right): left(left), right(right) {}

    const Expression *const left;
    const Expression *const right;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const {
        return "AdditionExpression(" + left->text() + "," + right->text() + ")";
    }
};

class SubtractionExpression: public Expression {
public:
    SubtractionExpression(const Expression *left, const Expression *right): left(left), right(right) {}
    const Expression *const left;
    const Expression *const right;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const {
        return "SubtractionExpression(" + left->text() + "," + right->text() + ")";
    }
};

class MultiplicationExpression: public Expression {
public:
    MultiplicationExpression(const Expression *left, const Expression *right): left(left), right(right) {}
    const Expression *const left;
    const Expression *const right;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const {
        return "MultiplicationExpression(" + left->text() + "," + right->text() + ")";
    }
};

class DivisionExpression: public Expression {
public:
    DivisionExpression(const Expression *left, const Expression *right): left(left), right(right) {}
    const Expression *const left;
    const Expression *const right;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const {
        return "DivisionExpression(" + left->text() + "," + right->text() + ")";
    }
};

class VariableReference {
public:
    virtual void generate(Emitter &emitter) const = 0;

    virtual std::string text() const = 0;
};

class ScalarVariableReference: public VariableReference {
public:
    ScalarVariableReference(const std::string &name): name(name) {}

    const std::string name;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const {
        return "ScalarVariableReference(" + name + ")";
    }
};

class FunctionReference: public VariableReference {
public:
    FunctionReference(const std::string &name): name(name) {}

    const std::string name;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const {
        return "FunctionReference(" + name + ")";
    }
};

class VariableExpression: public Expression {
public:
    VariableExpression(const VariableReference *var): var(var) {}

    const VariableReference *var;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const {
        return "VariableExpression(" + var->text() + ")";
    }
};

class FunctionCall: public Expression {
public:
    FunctionCall(const VariableReference *func, const std::vector<const Expression *> &args): func(func), args(args) {}

    const VariableReference *const func;
    const std::vector<const Expression *> args;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const;
};

class Statement: public AstNode {
public:
    virtual void generate(Emitter &emitter) const = 0;
};

class AssignmentStatement: public Statement {
public:
    AssignmentStatement(const VariableReference *variable, const Expression *expr): variable(variable), expr(expr) {}

    const VariableReference *const variable;
    const Expression *const expr;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const {
        return "AssignmentStatement(" + variable->text() + ", " + expr->text() + ")";
    }
};

class ExpressionStatement: public Statement {
public:
    ExpressionStatement(const Expression *expr): expr(expr) {}

    const Expression *const expr;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const {
        return "ExpressionStatement(" + expr->text() + ")";
    }
};

class CompoundStatement: public Statement {
public:
    CompoundStatement(const std::vector<const Statement *> &statements): statements(statements) {}

    const std::vector<const Statement *> statements;

    virtual void dumpsubnodes(int depth) const;
};

class IfStatement: public CompoundStatement {
public:
    IfStatement(const Expression *condition, const std::vector<const Statement *> &statements): CompoundStatement(statements), condition(condition) {}

    const Expression *condition;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const {
        return "IfStatement(" + condition->text() + ")";
    }
};

class WhileStatement: public CompoundStatement {
public:
    WhileStatement(const Expression *condition, const std::vector<const Statement *> &statements): CompoundStatement(statements), condition(condition) {}

    const Expression *condition;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const {
        return "WhileStatement(" + condition->text() + ")";
    }
};

class Program: public AstNode {
public:
    Program(const std::vector<const Statement *> &statements): statements(statements) {}

    const std::vector<const Statement *> statements;

    virtual void generate(Emitter &emitter) const;

    virtual std::string text() const { return "Program"; }
    virtual void dumpsubnodes(int depth) const;
};

#endif
