const {readFileSync} = require('fs');

const data = readFileSync(`./input.txt`).toString();
splitData = data.split('\n')

const spelledNumbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]

sum = 0
splitData.forEach((line) => {
    let numbers = line.split('').filter(char => /[0-9]/.test(char)).join('')
    let leftNumber = numbers.slice(0, 1)
    let rightNumber = numbers.slice(-1)


    let leftWord = ''
    let rightWord = ''

    spelledNumbers.forEach(num => {
        if (!line.includes(num)) {
            return
        }
        if(leftWord === '') {
            leftWord = num
        }
        if(line.indexOf(num) < line.indexOf(leftWord)) {
            leftWord = num
        }

    })

    spelledNumbers.forEach(num => {

        if (!line.includes(num)) {
            return
        }
        if(rightWord === '') {
            rightWord = num
        }
        if (line.lastIndexOf(num) > line.lastIndexOf(rightWord)) {
            rightWord = num
        }

    })

    if (line.indexOf(leftNumber) < line.indexOf(leftWord) || line.indexOf(leftWord) === -1) {
        finalLeft = leftNumber
    } else {
        finalLeft = (spelledNumbers.indexOf(leftWord) + 1).toString()
    }
    if (line.lastIndexOf(rightNumber) > line.lastIndexOf(rightWord) || line.lastIndexOf(rightWord) === -1) {
        finalRight = rightNumber
    } else {
        finalRight = (spelledNumbers.indexOf(rightWord) + 1).toString()
    }

    if (finalLeft == 0 && finalRight == 0) {
        console.log("WOW")
        let numbers = line.split('').filter(char => /[0-9]/.test(char)).join('')
        finalLeft = numbers.slice(0, 1)
        finalRight = numbers.slice(-1)
    }

    console.log(finalLeft + finalRight + ' ' + line)
    // console.log(leftNumber + rightNumber)
    sum += parseInt(finalLeft + finalRight)
})
console.log(sum)





