const std = @import("std");
const token = @import("token.zig");
const TokenKind = token.TokenKind;
const Token = token.Token;

pub const Lexer = struct {
    const Self = @This();

    input: []const u8,
    position: usize, // current position
    read_position: usize, // next position
    ch: u8,

    pub fn new(s: []const u8) Lexer {
        var l = Lexer{
            .input = s,
            .position = 0,
            .read_position = 0,
            .ch = 0b0,
        };
        l.readChar();
        return l;
    }

    pub fn readChar(self: *Self) void {
        if (self.read_position >= self.input.len) {
            self.ch = 0b0;
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn readIdentifier(self: *Self) Token {
        const position = self.position;

        while (std.ascii.isAlphabetic(self.ch)) {
            self.readChar();
        }

        return Token.new(TokenKind.IDENT, null, self.input[position..self.position]);
    }

    pub fn nextToken(self: *Self) Token {
        const tok = switch (self.ch) {
            '=' => Token.new(TokenKind.ASSIGN, null, null),
            ';' => Token.new(TokenKind.SEMICOLON, null, null),
            '(' => Token.new(TokenKind.LPAREN, null, null),
            ')' => Token.new(TokenKind.RPAREN, null, null),
            ',' => Token.new(TokenKind.COMMA, null, null),
            '+' => Token.new(TokenKind.PLUS, null, null),
            '{' => Token.new(TokenKind.LBRACE, null, null),
            '}' => Token.new(TokenKind.RBRACE, null, null),
            0b0 => Token.new(TokenKind.EOF, null, null),
            else => if (std.ascii.isAlphabetic(self.ch)) self.readIdentifier() else Token.new(TokenKind.ILLEGAL, null, null),
        };
        self.readChar();
        return tok;
    }
};

test "test next token" {
    const input = "=+(){},;";
    const test_arr = [_]Token{
        Token.new(TokenKind.ASSIGN, null, null),
        Token.new(TokenKind.PLUS, null, null),
        Token.new(TokenKind.LPAREN, null, null),
        Token.new(TokenKind.RPAREN, null, null),
        Token.new(TokenKind.LBRACE, null, null),
        Token.new(TokenKind.RBRACE, null, null),
        Token.new(TokenKind.COMMA, null, null),
        Token.new(TokenKind.SEMICOLON, null, null),
        Token.new(TokenKind.EOF, null, null),
    };

    var l = Lexer.new(input);

    for (test_arr) |tt| {
        const tok = l.nextToken();
        try std.testing.expectEqual(tok.ttype, tt.ttype);
        try std.testing.expectEqual(tok.literal, tt.literal);
    }
}
