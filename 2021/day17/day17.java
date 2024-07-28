
import java.io.File;
import java.util.Scanner;
import java.util.regex.*;

public class day17 {
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

    public static int triangularate(int n) {
        return n * (Math.abs(n) + 1) / 2;
    }

    public static int x_after_ticks(int initial_x, int ticks) {
        return triangularate(initial_x) - triangularate(Math.max(initial_x - ticks, 0));
    }

    public static int y_after_ticks(int initial_y, int ticks) {
        if (initial_y >= 0 && ticks <= initial_y)
            return x_after_ticks(initial_y, ticks); // same logic
        if (initial_y >= 0) // ticks > |initial_y|, ascend until y velocity is 0, then start descending
            return triangularate(initial_y) - triangularate(ticks - initial_y - 1);
        // initial_y < 0, get the triangular for faux full distance, remove triangular of everything skipped
        return triangularate(initial_y - ticks + 1) - triangularate(initial_y + 1);
    }

    public static Coordinate coordinate_after_ticks(Coordinate velocity, int ticks) {
        return new Coordinate(x_after_ticks(velocity.x, ticks), y_after_ticks(velocity.y, ticks));
    }

    public static boolean coordinate_within_target(Coordinate position) {
        return target_left <= position.x && position.x <= target_right 
            && target_up   >= position.y && position.y >= target_down;
    }

    public static boolean enters_target(int x, int y) {
        Coordinate here = null, velocity = new Coordinate(x, y);
        for (int tick = 1;; tick++) {
            here = coordinate_after_ticks(velocity, tick);
            if (coordinate_within_target(here)) return true;
            if (here.x > target_right || here.y < target_down) return false;
        }
    }

    public static int trajectories_that_enter_target() {
        // cheeky shorthand to trim impossible solutions
        int min_x, max_x, min_y, max_y;
        min_x = (int) Math.sqrt(target_left * 2.0) + 1; // minimum x needed to get within the band
        max_x = target_right; // x > target_right means first tick goes beyond the target and cannot come back
        min_y = target_down; // y < target_down means same as prev
        max_y = -target_down - 1; // upwards launches will hit y_pos=0, and on next tick descend initial_y_vel+1, so same as prev
        int found_velocities = 0;
        for (int y = min_y; y <= max_y; y++)
            for (int x = min_x; x <= max_x; x++)
                if (enters_target(x, y))
                    found_velocities++;
        return found_velocities;
    }

    private static int target_left, target_right, target_up, target_down;
    public static void main(String[] args) {
        try {
            Scanner input_scanner = new Scanner(new File("2021/day17/day17.txt"));
            Matcher input_data = Pattern.compile("^target area: x=(-?\\d*)..(-?\\d*), y=(-?\\d*)..(-?\\d*)$").matcher(input_scanner.nextLine());
            input_data.find(); input_scanner.close();
            target_left = Integer.parseInt(input_data.group(1));
            target_right = Integer.parseInt(input_data.group(2));
            target_down = Integer.parseInt(input_data.group(3));
            target_up = Integer.parseInt(input_data.group(4));

            // assumption: there is a triangular x velocity value within the target area
            int part_1_x = (int) Math.sqrt(target_right * 2.0);
            assert target_left <= part_1_x && part_1_x <= target_right;
            // if assumption is true, then tick=>+infinity means x remains in target x dimension
            // thus, find a y that is valid: positive y velocity eventually return to y=0 with -y_vel-1
            // can be simplified as the bottom of the target decreased by 1
            int part_1_y = -target_down - 1;
            System.out.println(enters_target(part_1_x, part_1_y));
            System.out.println(triangularate(part_1_y));

            System.out.println(trajectories_that_enter_target());

        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
