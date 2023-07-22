const std = @import("std");
const heap = std.heap;
const curses = @import("curses.zig");

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
    const window = try curses.initscr(ally);
    _ = window;
    try curses.start_color(); // Enable color support

    // Define color pairs
    const pair1 = try curses.ColorPair.init(1, curses.COLOR_RED, curses.COLOR_BLACK);
    _ = pair1;
    const pair2 = try curses.ColorPair.init(2, curses.COLOR_GREEN, curses.COLOR_BLACK);
    _ = pair2;

    // Print some text
    curses.mvprintw(5, 10, "Hello, colorful world!");

    // Apply attributes and color pair to specific characters
    curses.chgat(5, curses.A_BOLD | curses.A_UNDERLINE, 1, null); // Make the first 5 characters bold and underlined using color pair 1
    curses.chgat(3, curses.A_BOLD, 2, null); // Make the next 3 characters bold using color pair 2

    curses.refresh(); // Refresh the screen

    curses.getch(); // Wait for a key press
    curses.endwin(); // End curses mode

    // Prints to stderr (it's a shortcut based on `std.io.getStdErr()`)
    std.debug.print("All your {s} are belong to us.\n", .{"codebase"});

    // stdout is for the actual output of your application, for example if you
    // are implementing gzip, then only the compressed bytes should be sent to
    // stdout, not any debugging messages.
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Run `zig build test` to run the tests.\n", .{});

    try bw.flush(); // don't forget to flush!
}

test "simple test" {
    var list = std.ArrayList(i32).init(std.testing.allocator);
    defer list.deinit(); // try commenting this out and see if zig detects the memory leak!
    try list.append(42);
    try std.testing.expectEqual(@as(i32, 42), list.pop());
}
