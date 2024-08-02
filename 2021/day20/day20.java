
import java.io.File;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Scanner;
import java.util.Set;

public class day20 {
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
    private static List<Coordinate> kernel = Arrays.asList(
        new Coordinate(-1, -1), new Coordinate(0, -1), new Coordinate(1, -1), 
        new Coordinate(-1, 0), new Coordinate(0, 0), new Coordinate(1, 0), 
        new Coordinate(-1, 1), new Coordinate(0, 1), new Coordinate(1, 1)
    );
    private static List<Boolean> enhancement_algorithm;

    private static class Image {
        int left, right, top, bottom;
        HashSet<Coordinate> on_pixels;
        boolean outside = false;

        public Image(List<String> board) {
            on_pixels = new HashSet<>();
            for (int i = 0; i < board.size(); i++) {
                for (int j = 0; j < board.get(i).length(); j++) {
                    if (board.get(i).charAt(j) == '.') continue;

                    left = Math.min(left, j); right = Math.max(right, j);
                    top = Math.min(top, i); bottom = Math.max(bottom, i);
                    on_pixels.add(new Coordinate(j, i));
                }
            }
        }

        public boolean get_pixel_status(Coordinate pixel) {
            if (pixel.x < left || right < pixel.x || 
                pixel.y < top || bottom < pixel.y) return outside;
            else 
                return on_pixels.contains(pixel);
        }

        public boolean get_pixel_next_status(Coordinate pixel) {
            return enhancement_algorithm.get(
                Integer.parseInt(
                    String.join("", 
                        kernel.stream().map(offset -> this.get_pixel_status(offset.add(pixel)) ? "1" : "0").toList()
                    ), 2
                )
            );
        }

        public void enhance() {
            HashSet<Coordinate> next_on_pixel = new HashSet<>();
            int next_top = top, next_bottom = bottom, next_left = left, next_right = right;
            for (int i = top-1; i <= bottom+1; i++) {
                for (int j = left-1; j <= right+1; j++) {
                    Coordinate here = new Coordinate (j, i);
                    if (!get_pixel_next_status(here)) continue;
                    next_on_pixel.add(here);
                    next_left = Math.min(next_left, j); next_right = Math.max(next_right, j);
                    next_top = Math.min(next_top, i); next_bottom = Math.max(next_bottom, i);
                }
            }
            outside = enhancement_algorithm.get(Integer.parseInt(new String(new char[9]).replace("\0", outside ? "1" : "0"), 2));
            on_pixels = next_on_pixel;
            top = next_top; bottom = next_bottom; left = next_left; right = next_right;
        }
    }

    public static void main(String[] args) {
        try {
            Scanner input_scanner = new Scanner(new File("2021/day20/day20.txt"));
            enhancement_algorithm = input_scanner.next().chars().mapToObj(entry -> entry == '#').toList();

            List<String> board = new ArrayList<>();
            while (input_scanner.hasNext()) board.add(input_scanner.next());
            input_scanner.close();
            Image image = new Image(board);

            int enhance_pt1 = 2, enhance_pt2 = 50;
            for (int i = 0; i < enhance_pt1; i++)
                image.enhance();
            System.out.println(image.on_pixels.size());
            
            for (int i = enhance_pt1; i < enhance_pt2; i++)
                image.enhance();
            System.out.println(image.on_pixels.size());

        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
