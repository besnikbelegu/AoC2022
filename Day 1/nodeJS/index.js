const fs = require('fs/promises');
const fileName = 'data.txt';
async function readFileLocally(_file) {
    try {
        let dataFile = `${__dirname}/${_file}`;
        return await fs.readFile(dataFile, { encoding: 'utf-8' });
    } catch (error) {
        console.error(error)
    }

}

readFileLocally(fileName).then(data => {
    const groupedData = data.split(/\r?\n\n/)

    let elfs = [];
    groupedData.forEach(bulk => {
        let arr = bulk.split(/\r?\n/)
        let sum = arr.reduce((partialSum, current) => partialSum + parseInt(current), 0)
        elfs.push(sum);
    });

    // Part One
    const biggestValue = Math.max.apply(null, elfs);
    const ElfTeam = elfs.indexOf(biggestValue) + 1;
    console.log(`Biggest value is:${biggestValue} and it belogs to Elf Team ${ElfTeam}`)

    // Part Two
    const sortetElfs = elfs.sort((a, b) => a - b);
    const topThreeTeams = sortetElfs.slice(-3);
    const totalOfTopThree = topThreeTeams.reduce((partialSum, current) => partialSum + parseInt(current), 0)
    console.log(
        `Top three are: ${topThreeTeams}.
         The total of the top three teams is: ${totalOfTopThree}`)

    // console.log(data);
})