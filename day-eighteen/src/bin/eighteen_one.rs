fn main() {
    let (points, boundary_area) =
        include_str!("input.txt")
            .lines()
            .fold((vec![], 0), |(mut points, boundary_area), line| {
                let mut chunks = line.split_whitespace();
                let dir = chunks.next().unwrap();
                let n = chunks.next().unwrap().parse::<usize>().unwrap();

                let (dx, dy) = match dir {
                    "U" => (0, 1),
                    "D" => (0, -1),
                    "L" => (-1, 0),
                    "R" => (1, 0),
                    o => panic!("unexpected direction {o}"),
                };

                let (prev_x, prev_y) = points.last().unwrap_or(&(0, 0));
                points.push((prev_x + (dx * n as isize), prev_y + (dy * n as isize)));

                (points, boundary_area + n)
            });

    let area = shoelace_formula(&points);
    // pick's theorem
    let inner_area = area - (boundary_area / 2) + 1;
    let solution = inner_area + boundary_area;

    println!("{solution}")
}

fn shoelace_formula(points: &[(isize, isize)]) -> usize {
    let area = points.iter().enumerate().fold(0, |area, (i, (x1, y1))| {
        let (x2, y2) = points[i.checked_sub(1).unwrap_or(points.len() - 1)];

        area + (x1 * y2) - (x2 * y1)
    });

    (area.abs() / 2) as usize
}
