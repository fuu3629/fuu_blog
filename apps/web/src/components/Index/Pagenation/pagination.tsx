import ReactPaginate from "react-paginate";

export interface PaginationProps {
  page?: number;
  pageSize: number;
  totalCount: number;
  onChange: (page: number) => void;
}

export default function Pagination({
  page,
  pageSize,
  totalCount,
  onChange,
}: PaginationProps) {
  const pageCount = Math.ceil(totalCount / pageSize);
  return (
    <ReactPaginate
      pageCount={pageCount}
      breakLabel="..."
      nextLabel="next >"
      previousLabel="< previous"
      pageRangeDisplayed={5}
      onPageChange={(selectedItem) => onChange(selectedItem.selected + 1)}
    ></ReactPaginate>
  );
}
