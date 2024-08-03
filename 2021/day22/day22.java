
import java.util.regex.Pattern;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;

public class day22 {
    private static class Cube {
        int xs, xl, ys, yl, zs, zl;
        int mult; // 'turn on cube' -> 1, 'turn off cube (starter)' -> 0, 'turn on cube (found intersect)' -> -1

        public Cube(String raw_input) {
            Matcher results = Pattern.compile("^(\\w+) x=(-?\\d+)\\.\\.(-?\\d+),y=(-?\\d+)\\.\\.(-?\\d+),z=(-?\\d+)\\.\\.(-?\\d+)$").matcher(raw_input);
            results.find();
            this.mult = results.group(1).equals("on") ? 1 : 0;
            this.xs = Integer.parseInt(results.group(2));
            this.xl = Integer.parseInt(results.group(3));
            this.ys = Integer.parseInt(results.group(4));
            this.yl = Integer.parseInt(results.group(5));
            this.zs = Integer.parseInt(results.group(6));
            this.zl = Integer.parseInt(results.group(7));
        }
        public Cube(int mult, int x_lower, int x_higher, int y_lower, int y_higher, int z_lower, int z_higher) {
            this.mult = mult;
            this.xs = x_lower; this.xl = x_higher;
            this.ys = y_lower; this.yl = y_higher;
            this.zs = z_lower; this.zl = z_higher;
        }

        public boolean does_intersect(Cube that) {
            // if larger of the smalls is at or lower than smaller of the larges then that axis overlaps
            // if all axis overlap, cubes overlap
            return Math.max(this.xs, that.xs) <= Math.min(this.xl, that.xl) 
                && Math.max(this.ys, that.ys) <= Math.min(this.yl, that.yl)
                && Math.max(this.zs, that.zs) <= Math.min(this.zl, that.zl);
        }
        public Cube intersect(Cube that) {
            // assumption: you aren't calling this if there isn't a valid intersection
            // assumption: intersection likewise cubes => intersection should be opposite
            //    on A intersect on B => off A&B
            return new Cube(-that.mult, 
                Math.max(this.xs, that.xs), Math.min(this.xl, that.xl), 
                Math.max(this.ys, that.ys), Math.min(this.yl, that.yl),
                Math.max(this.zs, that.zs), Math.min(this.zl, that.zl));
        }

        public int volume() {
            return mult * (xl - xs + 1) * (yl - ys + 1) * (zl - zs + 1);
        }

        public String toString() {
            return String.format("%d x=%d %d, y=%d %d, z=%d %d", mult, xs, xl, ys, yl, zs, zl);
        }
    }

    public static int get_on_cubes(List<Cube> cubes) {
        // construct list of all chunks to enable/disable
        List<Cube> core_chunks = new ArrayList<>();
        for (Cube cube : cubes) {
            ArrayList<Cube> new_chunks = new ArrayList<>();
            new_chunks.add(cube);

            // for existing chunk, add intersects
            // first chunk A means chunks=[A]
            // second chunk B is chunks=[A,B,-A&B], if B is turn off lights then 0 volume, so A - A&B
            // third chunk C is chunks=[A,B,-A&B,C,-A&C,-B&C,+A&B&C]
            for (Cube chunk : core_chunks)
                if (cube.does_intersect(chunk))
                    new_chunks.add(cube.intersect(chunk));
            core_chunks.addAll(new_chunks);
        }

        // count squares based on chunks
        // on chunk is +, off chunk (intersect) is -
        return core_chunks.stream().mapToInt(Cube::volume).sum();
    }

    public static void main(String[] args) {
        try {
            List<Cube> raw_cubes = Files
                    .lines(Paths.get("2021/day22/day22.txt"))
                    .map(x -> new Cube(x))
                    .toList();
            
            Cube activation_zone = new Cube(0, -50, 50, -50, 50, -50, 50);
            List<Cube> part_1_cubes = raw_cubes.stream().filter(x -> x.does_intersect(activation_zone)).map(x -> activation_zone.intersect(x)).toList();
            part_1_cubes.forEach(x -> x.mult *= -1);
            System.out.println(get_on_cubes(part_1_cubes));
            
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
