
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

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

class Line {
    public Coordinate start;
    public Coordinate delta;

    public Line(Coordinate start, Coordinate end) {
        this.start = start;
        this.delta = end.sub(start);
    }
    
    public boolean cardinal() {
        return (delta.x * delta.y) == 0;
    }

    public List<Coordinate> get_points() {
        Coordinate step_increment = new Coordinate(delta.x == 0 ? 0 : delta.x / Math.abs(delta.x), delta.y == 0 ? 0 : delta.y / Math.abs(delta.y));
        int step_counts = Math.max(Math.abs(delta.x), Math.abs(delta.y));
        
        List<Coordinate> result = new ArrayList<>();
        Coordinate location = start;
        result.add(location);
        for (int step = 0; step < step_counts; step++) {
            location = location.add(step_increment);
            result.add(location);
        }
        return result;
    }

    public String toString() {
        return String.format("%s -> %s", this.start.toString(), this.start.add(this.delta).toString());
    }
}

public class day05 {
    public static Coordinate make_coordinate(String input_string) {
        String[] dimension_strings = input_string.split(",");
        return new Coordinate(
            Integer.parseInt(dimension_strings[0]), 
            Integer.parseInt(dimension_strings[1]));
    }

    public static Line make_line(String input_string) {
        String[] coord_strings = input_string.split(" -> ");
        return new Line(make_coordinate(coord_strings[0]), make_coordinate(coord_strings[1]));
    }

    public static void main(String[] args) {
        try {
            // input file -> list of Integer
            List<Line> data = Files
                    .lines(Paths.get("2021/day05/day05.txt"))
                    .map(x -> make_line(x))
                    .toList();
            
            List<Line> cardinals = data.stream().filter(Line::cardinal).toList();
            Map<Coordinate, Integer> positions = new HashMap<>();
            for (Line line : cardinals) {
                List<Coordinate> points = line.get_points();
                for (Coordinate point : points) {
                    if (!positions.containsKey(point))
                        positions.put(point, 0);
                    positions.put(point, positions.get(point) + 1);
                }
            }
            int multi_points = positions.entrySet().stream().mapToInt(x -> x.getValue() > 1 ? 1 : 0).sum();
            System.out.println(multi_points);

        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
