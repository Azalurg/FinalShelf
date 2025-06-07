interface Book {
  title: string;
  relative_cover_path: string;
  author_name: string;
  genre: string;
  lector: string;
  create_date: string;
  read: boolean;
  score: number;
}

// interface BookDetails extends Book {
//     duration: number,
//     year: number,
//     genre_id: number,
//     genre_name: string,
//     author_picture_path: string,
//     lector_id: number,
//     lector_name: string,
// }

interface BookListResponse {
  books: Book[];
  total_count: number;
  page: number;
  limit: number;
  total_pages: number;
}

export { Book, BookListResponse };
