
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Set;
import java.util.function.Function;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class day11 {
    private static class Coordinate {
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
    private static int[][] data;
    private static int width, height;
    private static List<Coordinate> neighbours = Arrays.asList(
        new Coordinate(0, 1), new Coordinate(0, -1), new Coordinate(1, 0), new Coordinate(-1, 0),
        new Coordinate(1, 1), new Coordinate(1, -1), new Coordinate(-1, 1), new Coordinate(-1, -1));

    public static boolean is_valid_coord(Coordinate coord) {
        return 0 <= coord.x && coord.x < width && 0 <= coord.y && coord.y < height;
    }

    public static Integer read_coord(Coordinate coord) {
        return is_valid_coord(coord) ? data[coord.y][coord.x] : null;
    }

    public static void write_coord(Coordinate coord, int value) {
        if (is_valid_coord(coord))
            data[coord.y][coord.x] = value;
    }

    public static void inc_coord(Coordinate coord) {
        if (is_valid_coord(coord))
            write_coord(coord, read_coord(coord) + 1);
    }

    public static int step() {
        IntStream.range(0, height)
                .mapToObj(y -> IntStream.range(0, width)
                    .mapToObj(x -> new Coordinate(x, y)))
                .flatMap(Function.identity()) // stream of all coordinates in space
                .forEach(coord -> inc_coord(coord));

        LinkedList<Coordinate> to_flash = IntStream.range(0, height)
                .mapToObj(y -> IntStream.range(0, width)
                    .mapToObj(x -> new Coordinate(x, y)))
                .flatMap(Function.identity()) // stream of all coordinates in space
                .filter(coord -> read_coord(coord) > 9) // initial list of all coordinates that will flash
                .collect(Collectors.toCollection(LinkedList::new));
        Set<Coordinate> flashed = new HashSet<>();
        while (to_flash.size() > 0) {
            final Coordinate here = to_flash.pop();
            if (flashed.contains(here)) // if we've already visited here do nothing
                continue;
            flashed.add(here);

            neighbours.stream().map(coord -> coord.add(here))
                .forEach(coord -> inc_coord(coord)); // increment all neighbours

            neighbours.stream().map(coord -> coord.add(here))
                .filter(coord -> is_valid_coord(coord))
                .filter(coord -> read_coord(coord) > 9) // new coordinates to flash
                .forEach(coord -> to_flash.push(coord)); // add them to checklist
        }
        flashed.forEach(coord -> write_coord(coord, 0)); // set all flashed coords to 0
        return flashed.size();
    }

    public static void print_data() {
        for (int y = 0; y < height; y++) {
            for (int x = 0; x < width; x++) {
                System.out.print(data[y][x]);
            }
            System.out.println();
        }
        System.out.println();
    }
    
    public static void main(String[] args) {
        try {
            data = Files
                    .lines(Paths.get("2021/day11/day11.txt"))
                    .map(y -> y.chars().map(x -> (x - '0')).toArray())
                    .toArray(int[][]::new);
            width = data[0].length; height = data.length;

            int cycles = 1;
            int result = 0;
            for (; cycles <= 100; cycles++)
                result += step();
            System.out.println(result);
            
            while (step() < height * width)
                cycles++;
            System.out.println(cycles);

        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
