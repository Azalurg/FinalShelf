import { Book, ListResponse } from "./books";

interface Series {
  id: number;
  name: string;
  author_name: string;
  description: string | null;
}

interface SeriesListItem {
  id: number;
  name: string;
  author_name: string;
  description: string | null;
  books_count: number;
}

interface SeriesWithBooks {
  series: Series;
  books: Book[];
}

type SeriesListResponse = ListResponse<SeriesListItem>;

export { Series, SeriesListItem, SeriesWithBooks, SeriesListResponse };
