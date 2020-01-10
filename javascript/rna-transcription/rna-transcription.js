//
// This is only a SKELETON file for the 'RNA Transcription' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export const toRna = sequence => {
  var transcription = '';

 if(transcription === sequence)
  return transcription;

  for(let i = 0; i < sequence.length; i++){
    if(sequence[i] === 'G'){
      transcription += 'C';
    }
    else if(sequence[i] === 'C'){
      transcription += 'G';
    }
    else if(sequence[i] === 'T'){
      transcription += 'A';
    }
    else if(sequence[i] === 'A'){
      transcription += 'U';
    }
    else{
      transcription += sequence[i];
    }
  }

  return transcription;
};
