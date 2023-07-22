// Katz, a command-line chat client for
// the Sparklet Katz protocol
// --- by &Cherry and III_zP0_III (npxTSC)
//
// Used this repo for Zig ncurses boilerplate:
// https://github.com/Akuli/curses-minesweeper
// <3 Open Source

const std = @import("std");
const curses = @import("curses.zig");
const websocket = @import("websocket");
const ws = @import("socket.zig");
const heap = std.heap;

const SRV_ADDR = "127.0.0.1"; // must be an IP address... TODO comptime dns resolution?
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

    // this is the instance of your "global" struct to pass into your handlers
    var context = ws.Context{};

    try websocket.listen(ws.Handler, ally, &context, .{
        .address = SRV_ADDR,
        .port = SRV_PORT,
        .max_headers = 10,
    });

    // Define color pairs
    const pair1 = try curses.ColorPair.init(1, curses.COLOR_RED, curses.COLOR_BLACK);

    // Print some text
    try win.attron(pair1.attr());
    try win.mvaddstr(5, 10, "Hello, colorful world!");

    _ = try win.getch(); // Wait for a key press
    _ = try curses.endwin(); // End curses mode
}
