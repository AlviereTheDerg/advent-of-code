
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Set;

public class day15 {
    private static class Coordinate {
        public final int x;
        public final int y;
    
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
        new Coordinate(0, 1), new Coordinate(0, -1), new Coordinate(1, 0), new Coordinate(-1, 0));
    
    private static interface CoordReader {
        public boolean is_valid_coord(Coordinate coord);
        public Integer read_coord(Coordinate coord);
    }

    private static class NormalCoord implements CoordReader {
        public boolean is_valid_coord(Coordinate coord) {
            return 0 <= coord.x && coord.x < width && 0 <= coord.y && coord.y < height;
        }
    
        public Integer read_coord(Coordinate coord) {
            return is_valid_coord(coord) ? data[coord.y][coord.x] : null;
        }
    }

    private static class ExpandedCoord implements CoordReader {
        public boolean is_valid_coord(Coordinate coord) {
            return 0 <= coord.x && coord.x < 5*width && 0 <= coord.y && coord.y < 5*height;
        }
    
        public Integer read_coord(Coordinate coord) {
            if (!is_valid_coord(coord)) return null;
            int read_result = data[coord.y % height][coord.x % width];
            read_result += (coord.y / height) + (coord.x / width);
            while (read_result > 9) read_result -= 9;
            return read_result;
        }
    }

    private static class DistanceToken implements Comparable<DistanceToken> {
        public final Coordinate location;
        public final int distance;

        public DistanceToken(Coordinate location, int distance) {
            this.location = location;
            this.distance = distance;
        }

        public int compareTo(DistanceToken other) {
            return this.distance - other.distance;
        }
    };

    public static int dijkstra(CoordReader reader, Coordinate start, Coordinate end) {
        Set<Coordinate> expanded = new HashSet<>();
        PriorityQueue<DistanceToken> queue = new PriorityQueue<>();
        queue.add(new DistanceToken(start, 0));

        DistanceToken current;
        while ((current = queue.poll()) != null) {
            if (expanded.contains(current.location)) continue;
            expanded.add(current.location);
            
            for (Coordinate neighbour : neighbours) {
                neighbour = neighbour.add(current.location);
                if (!reader.is_valid_coord(neighbour)) continue;
                int neighbour_dist = current.distance + reader.read_coord(neighbour);
                if (end.equals(neighbour)) return neighbour_dist;
                queue.add(new DistanceToken(neighbour, neighbour_dist));
            }
        }
        return -1;
    }
    
    public static void main(String[] args) {
        try {
            data = Files
                    .lines(Paths.get("2021/day15/day15.txt"))
                    .map(y -> y.chars().map(x -> (x - '0')).toArray())
                    .toArray(int[][]::new);
            width = data[0].length; height = data.length;

            System.out.println(dijkstra(new NormalCoord(), new Coordinate(0, 0), new Coordinate(width-1, height-1)));
            System.out.println(dijkstra(new ExpandedCoord(), new Coordinate(0, 0), new Coordinate(5*width-1, 5*height-1)));
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
