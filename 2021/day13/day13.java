
import java.io.File;
import java.util.Arrays;
import java.util.List;
import java.util.OptionalInt;
import java.util.Scanner;
import java.util.Set;
import java.util.stream.Collectors;

public class day13 {
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

    private static class FoldToken {
        public final boolean vertical;
        public final int line;

        public FoldToken(String parameters) {
            String[] data = parameters.split(" ")[2].split("=");
            vertical = data[0].equals("y");
            line = Integer.parseInt(data[1]);
        }

        public String toString() {
            return String.format("%s=%d", (vertical ? "y" : "x"), line);
        }

        public Coordinate enact_fold(Coordinate inputCoordinate) {
            int judged = vertical ? inputCoordinate.y : inputCoordinate.x;
            if (judged <= line)
                return inputCoordinate;
            
            judged = 2*line - judged;
            if (vertical)
                return new Coordinate(inputCoordinate.x, judged);
            else
                return new Coordinate(judged, inputCoordinate.y);
        }

        public Set<Coordinate> enact_fold(Set<Coordinate> inputCoordinates) {
            return inputCoordinates.stream().map(x -> this.enact_fold(x)).collect(Collectors.toSet());
        }
    }

    public static void print_dots(Set<Coordinate> dots) {
        int max_x = dots.stream().mapToInt(coord -> coord.x).max().getAsInt(), 
            max_y = dots.stream().mapToInt(coord -> coord.y).max().getAsInt();
        
        for (int row_index = 0; row_index <= max_y; row_index++) {
            final int row_id = row_index;
            Set<Integer> column_vals = dots.stream().filter(coord -> coord.y == row_id).map(coord -> coord.x).collect(Collectors.toSet());
            for (int column_index = 0; column_index <= max_x; column_index++)
                System.out.print(column_vals.contains(column_index) ? "#" : ".");
            System.out.println();
        }
    }

    public static void main(String[] args) {
        try {
            File input_file = new File("2021/day13/day13.txt");
            Scanner input_scanner = new Scanner(input_file);
            input_scanner.useDelimiter("\n\n");

            Set<Coordinate> dots = Arrays.stream(input_scanner.next().split("\\n"))
                .map(x -> x.split(","))
                .map(x -> new Coordinate(Integer.parseInt(x[0]),Integer.parseInt(x[1])))
                .collect(Collectors.toSet());
            List<FoldToken> folds = Arrays.stream(input_scanner.next().split("\\n"))
                .map(x -> new FoldToken(x))
                .toList();
            input_scanner.close();

            System.out.println(folds.get(0).enact_fold(dots).size());
            for (FoldToken fold : folds)
                dots = fold.enact_fold(dots);
            print_dots(dots);
            
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
