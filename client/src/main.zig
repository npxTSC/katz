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

// ok ignore that comment i put here earlier...
// source: i'm a certified dumbass after 4AM
const SRV_ADDR = "127.0.0.1";
const SRV_PORT = 9098; // so serious :3

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

    while (true) {
        // Print some text
        try win.attron(pair1.attr());
        try win.mvaddstr(1, 2, "Hello, colorful world!");
        try win.boxme();

        const ch = try win.getch(); // Wait for a key press

        switch (ch) {
            'q' => {
                break;
            },

            else => {},
        }
    }

    _ = try curses.endwin(); // End curses mode
}
