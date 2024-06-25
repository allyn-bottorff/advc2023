const std = @import("std");

const number_table = [_]u8{'0', '1', '2', '3', '4', '5', '6', '7', '8', '9'};

pub fn main() !void {

    const cwd = std.fs.cwd();
    const file_string = []u8 = try cwd.readFile(, buffer: []u8)

    const openFlags: std.fs.File.OpenFlags = .{
        mode: .read_only,
        lock: .none,
    };

    const dir = std.fs.openFileAbsolute("./input.txt", .{OpenMode: .read_only})

    const file = std.fs.Dir.openFile

}

fn getNumberFromChar(c: u8) ?u8 {
    for (number_table, 0..) |number, i| {
        if (c == number) {
            return @truncate(i);
        }
    }
    return null;
}

fn getNumbersFromString(string: []const u8) [2]u8 {

    var numbers: [2]u8 = .{0, 0};

    for (string) |char| {
        const numOption = getNumberFromChar(char);
        if (numOption) |num| {
            numbers[0] = num;
            break;
        }
    }
    var i = string.len - 1;
    while (i > 0) : (i -= 1) {
        const numOption = getNumberFromChar(string[i]);
        if (numOption) |num| {
            numbers[1] = num;
            break;
        }
    }
    return numbers;
}


test "Get a 0 from getNumber" {
    if (getNumberFromChar('0')) |num| {
        try std.testing.expect(num == 0);
    } else {
        try std.testing.expect(false);
    }
}
test "Get a 9 from getNumber" {
    if (getNumberFromChar('9')) |num| {
        try std.testing.expect(num == 9);
    } else {
        try std.testing.expect(false);
    }
}

test "Get a non-digit from getNumber" {
    if (getNumberFromChar('a')) |_| {
        unreachable; 
    } else {
        try std.testing.expect(true);
    }
}

test "Get first and last number" {
    const string: []const u8 = &.{'a','1','s','d','f','j','k','l','j','2','l','k'};
    const numbers = getNumbersFromString(string);
    try std.testing.expect(numbers[0] == 1);
    try std.testing.expect(numbers[1] == 2);
}

test "Get first and last number 2" {
    const string: []const u8 = &.{'a','3','4','d','f','j','k','l','j','2','9','k'};
    const numbers = getNumbersFromString(string);
    try std.testing.expect(numbers[0] == 3);
    try std.testing.expect(numbers[1] == 9);
}

test "Get first and last number when there is only one number" {
    const string: []const u8 = &.{'a','3','o','d','f','j','k','l','j','d','k','k'};
    const numbers = getNumbersFromString(string);
    try std.testing.expect(numbers[0] == 3);
    try std.testing.expect(numbers[1] == 3);
}

test "Get first and last number when there are no numbers" {
    const string: []const u8 = &.{'a', 'b', 'c'};
    const numbers = getNumbersFromString(string);
    try std.testing.expect(numbers[0] == 0);
    try std.testing.expect(numbers[0] == 0);
}





