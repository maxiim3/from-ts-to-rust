// @ts-expect-error
import fs from 'fs'

type Line = string
type Index = number

fs
   .readFileSync('file.txt')
   .toString()
   .split('\n')
   .filter((line: Line, index: Index) => index % 2 === 0 && line)
   .splice(2, 2)
   .forEach((line: Line) => console.log(line))

