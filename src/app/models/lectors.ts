interface Lector {
  name: string;
  books_amount: number;
}

interface LectorDetails {
  name: string;
  books: string[];
  books_amount: number;
}

export { Lector, LectorDetails };
