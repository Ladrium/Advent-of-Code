
module.exports = data => {
    const fuel = (i) => {
        const f = Math.max(Math.floor(i / 3) - 2, 0);
        return f + (f > 0 ? fuel(f) : 0);
    };

    console.log(data.map(fuel).reduce((s, n) => s + n, 0));
}
