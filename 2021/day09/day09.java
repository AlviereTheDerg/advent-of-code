
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Queue;
import java.util.Set;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

class Coordinate {
    public int x;
    public int y;

    public Coordinate(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public Coordinate add(Coordinate other) {
        return new Coordinate(this.x + other.x, this.y + other.y);
    }

    public Coordinate sub(Coordinate other) {
        return new Coordinate(this.x - other.x, this.y - other.y);
    }

    public boolean equals(Object other) {
        if (other.getClass() != Coordinate.class)
            return false;
        
        Coordinate cast_other = (Coordinate) other;
        return (this.x == cast_other.x) && (this.y == cast_other.y);
    }
    public int hashCode() {
        return (this.x << 15) + this.y;
    }

    public String toString() {
        return String.format("(%d,%d)", this.x, this.y);
    }
}

public class day09 {
    private static int[][] data;
    private static int width, height;
    private static List<Coordinate> neighbours = Arrays.asList(new Coordinate(0, 1), new Coordinate(0, -1), new Coordinate(1, 0), new Coordinate(-1, 0));

    public static boolean is_valid_coord(Coordinate coord) {
        return 0 <= coord.x && coord.x < width && 0 <= coord.y && coord.y < height;
    }

    public static boolean low_point(Coordinate here_coord) {
        int here_value = data[here_coord.y][here_coord.x];
        return neighbours.stream() // the neighbour offsets
            .map(x -> x.add(here_coord)) // map them to the associated coordinates
            .filter(coord -> is_valid_coord(coord)) // strip those out of bounds
            .mapToInt(coord -> data[coord.y][coord.x]) // convert to the heights at those points
            .allMatch(x -> x > here_value); // assert that all of those values are greater than here
    }

    public static int expand_basin(Coordinate nucleation) {
        Queue<Coordinate> processing = new LinkedList<>();
        processing.add(nucleation);
        Set<Coordinate> visited = new HashSet<>();
        while (!processing.isEmpty()) {
            final Coordinate here = processing.remove();
            if (visited.contains(here) || !is_valid_coord(here) || data[here.y][here.x]==9)
                continue; // if already visited, OOB, or an impermeable wall
            
            visited.add(here);
            processing.addAll(neighbours.stream().map(x -> x.add(here)).toList());
        }
        return visited.size();
    }

    public static void main(String[] args) {
        try {
            data = Files
                    .lines(Paths.get("2021/day09/day09.txt"))
                    .map(y -> y.chars().map(x -> (x - '0')).toArray())
                    .toArray(int[][]::new);
            width = data[0].length; height = data.length;

            List<Coordinate> low_points = IntStream.range(0, height) // for y from 0 inc to height exc
                .mapToObj(y -> IntStream.range(0, width) // for x from 0 inc to width exc
                    .mapToObj(x -> new Coordinate(x, y))) // make stream of coordinates
                .flatMap(Function.identity()) // flatten to single stream of coordinates
                .filter(x -> low_point(x)) // filter to low points
                .toList();
            
            System.out.println(low_points.stream()
                .mapToInt(coord -> data[coord.y][coord.x] + 1) // extract low point's height from coord (and add 1)
                .sum());

            List<Integer> basin_sizes = low_points.stream().map(x -> expand_basin(x)).collect(Collectors.toList());
            basin_sizes.sort(null);
            System.out.println(basin_sizes.get(basin_sizes.size() - 3) * basin_sizes.get(basin_sizes.size() - 2) * basin_sizes.get(basin_sizes.size() - 1));
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
