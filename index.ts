enum Color {
   Red,
   Green,
   Blue,
   Yellow
}

function printColor(color: Color) {
   switch (color) {
      case Color.Blue:
         console.log('blue')
         break
      case Color.Red:
         console.log('red')
         break
      case Color.Green:
         console.log('green')
         break
   }
}

printColor(Color.Yellow)
