const std = @import("std");
const Path = std.fs.path;

pub const TokenKind = enum {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT,
    INT,

    // Operators
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
};

pub const Location = struct {
    row: usize,
    col: usize,
    file: Path,

    pub fn new(row: usize, col: usize, file: *Path) Location {
        return Location{
            .row = row,
            .col = col,
            .file = file,
        };
    }
};

pub const Token = struct {
    ttype: TokenKind,
    literal: []const u8,
    local: ?Location,

    pub fn new(ttype: TokenKind, local: ?Location, inner: ?[]const u8) Token {
        const literal = switch (ttype) {
            .EOF => "",
            .ILLEGAL => "ILLEGAL",
            .IDENT => inner orelse "_",
            .INT => inner orelse "0",
            .ASSIGN => "=",
            .PLUS => "+",
            .COMMA => ",",
            .SEMICOLON => ";",
            .LPAREN => "(",
            .RPAREN => ")",
            .LBRACE => "{",
            .RBRACE => "}",
            .FUNCTION => "fn",
            .LET => "let",
        };

        return Token{ .ttype = ttype, .literal = literal, .local = local };
    }
};
