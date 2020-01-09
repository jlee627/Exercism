//
// This is only a SKELETON file for the 'Resistor Color Duo' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const decodedValue = colorArray => {
  var colorValue = ["black","brown","red","orange","yellow","green","blue","violet","grey","white"];
  return parseInt(`${colorValue.indexOf(colorArray[0])}${colorValue.indexOf(colorArray[1])}`); 

};
