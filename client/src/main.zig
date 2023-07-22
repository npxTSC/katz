// Katz, a command-line chat client for
// the Sparklet Katz protocol
// --- by &Cherry and III_zP0_III (npxTSC)
//
// Used this repo for Zig ncurses boilerplate:
// https://github.com/Akuli/curses-minesweeper
// <3 Open Source

const std = @import("std");
const curses = @import("curses.zig");
const heap = std.heap;

pub fn main() !void {
    var gpa = heap.GeneralPurposeAllocator(.{}){};
    defer {
        const st = gpa.deinit();
        if (st == .leak) {
            std.debug.print("leaked (bruh)", .{});
        }
    }

    const ally = gpa.allocator();

    // Initialize the curses library
    const win = try curses.initscr(ally);
    try curses.start_color(); // Enable color support

    // Define color pairs
    const pair1 = try curses.ColorPair.init(1, curses.COLOR_RED, curses.COLOR_BLACK);

    // Print some text
    try win.attron(pair1.attr());
    try win.mvaddstr(5, 10, "Hello, colorful world!");

    _ = try win.getch(); // Wait for a key press
    _ = try curses.endwin(); // End curses mode
}
