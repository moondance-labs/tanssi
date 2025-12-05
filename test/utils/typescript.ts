export async function sleep(ms: number) {
    await new Promise((resolve) => setTimeout(resolve, ms));
}

// Escape a literal string so it can be matched using a regex without having to worry about special characters
// https://stackoverflow.com/a/3561711
export function escapeRegex(string) {
    return string.replace(/[/\-\\^$*+?.()|[\]{}]/g, "\\$&");
}
