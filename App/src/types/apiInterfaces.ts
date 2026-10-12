export interface IFirma {
    id: number;
    bezeichnung: string;
}

enum Stellenbezeichnung {
    Bearbeiter = 'Bearbeiter*in',
    Projektleiter = 'Projektleiter*in',
    Techniker = 'Techniker*in',
    Ingenieur = 'Ingenieur*in',
    Geschaeftsleiter = 'Geschäftsleiter*in',
    Praktikant = 'Praktikant*in / Azubi',
    Extern = 'Extern',
    IT = 'IT'
}

export interface IPerson {
    id: number;
    name: string;
    vorname: string;
    stellenbezeichnung:  Stellenbezeichnung | string;
    firma: number | null;  // Firma ID
    email: string;
    telefon: string;
}

export interface IProjekt {
    id: number;
    projektnummer: string;
    name: string;
    bezeichnung: string;
    start: string | null;
    ende: string | null;
}

export type Bauweise = 'Holz' | 'Stahl' | 'Stahlbeton' | 'Stahlverbund' | 'Naturstein' | 'Spannbeton' | '';

export interface IBauwerk {
    id: number;
    projekt: number[];  // Projekt IDs
    bezeichnung: string;
    bauwerksnummer: string;
    bauwerkstyp: string;
    bauweise: Bauweise;
    beschreibung: string;
    bemerkung: string;
    strasse: string;
    archived: boolean;
}

// The Untersuchungsstelle references the Bauwerk directly.
export interface IUntersuchungsstelle {
    id: number;
    bauwerk: number;
    projekt: number | null;
    name: string;
    bemerkung: string;
    lagenbezugHoehe: string;
    hoehe: number | null;
    lagenbezugAbstand1: string;
    abstand1: number | null;
    lagenbezugAbstand2: string;
    abstand2: number | null;
    media?: number[];
}
