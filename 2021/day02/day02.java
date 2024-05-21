
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;
import java.util.function.Predicate;

public class day02 {
    private static class Move {
        boolean vertical;
        int distance;

        public Move(String line) {
            String[] args = line.split(" ");
            this.vertical = (args[0].equals("up") || args[0].equals("down"));
            this.distance = Integer.parseInt(args[1]);
            if (args[0].equals("up"))
                this.distance *= -1;
        }

        public boolean getVertical() {
            return this.vertical;
        }
        public int getDistance() {
            return this.distance;
        }

        public String toString() {
            return (vertical ? "vertical " : "forward ") + distance;
        }
    }
    public static void main(String[] args) {
        try {
            // input file -> list of Integer
            List<Move> data = Files
                    .lines(Paths.get("2021/day02/day02.txt"))
                    .map(x -> new Move(x))
                    .toList();

            int depth = data.stream().filter(Move::getVertical).mapToInt(Move::getDistance).sum();
            int horizontal = data.stream().filter(Predicate.not(Move::getVertical)).mapToInt(Move::getDistance).sum();
            System.out.println(depth * horizontal);
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
