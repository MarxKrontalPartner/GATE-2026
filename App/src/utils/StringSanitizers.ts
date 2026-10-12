
// replace ae, oe, ue with ä, ö, ü
function replaceLettersWithUmlauts(inputString: string): string {
    return inputString.replace('ae', 'ä').replace('oe', 'ö').replace('ue', 'ü');
  }

// other than for umlauts, you can't simply replace 'ss' with 'ß' in a string
// so we need to define the words that should be replaced
function replaceWithDisplayName(inputString: string): string {

    const names: { [key: string]: string }  = {
      'ausreisser': 'Ausreißer',
      'massnahmen': 'Maßnahmen',
      'it-infrastruktur': 'IT-Infrastruktur',
      'export-preprocessing': 'Export Preprocessing',
      'data-lake-tabellen': 'Data-Lake-Tabellen',
    };
  
    if (names[inputString.toLowerCase()]) {
      return names[inputString.toLowerCase()];
    }
    return inputString;
  
  }

export { replaceLettersWithUmlauts, replaceWithDisplayName };