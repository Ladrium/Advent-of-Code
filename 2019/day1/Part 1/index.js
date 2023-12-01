module.exports = (data) => {
    const calc = (el) => Math.floor(el/3) -2;
    console.log(data.map(calculate).reduce((a, b) => a+b));
}