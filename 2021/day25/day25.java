
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.HashSet;
import java.util.Iterator;
import java.util.List;

public class day25 {
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

    private static class CucumberSwarm {
        int right, bottom;
        HashSet<Coordinate> easts, souths, inactive_easts, inactive_souths;
        static final Coordinate east_movement = new Coordinate(1, 0), south_movement = new Coordinate(0, 1);

        public CucumberSwarm(List<String> raw_input) {
            bottom = raw_input.size(); right = raw_input.get(0).length();
            HashSet<Coordinate> to_sort_easts = new HashSet<>(), to_sort_souths = new HashSet<>();
            for (int i = 0; i < raw_input.size(); i++) {
                for (int j = 0; j < raw_input.get(i).length(); j++) {
                    switch (raw_input.get(i).charAt(j)) {
                        case '.': break;
                        case '>': to_sort_easts.add(new Coordinate(j,i)); break;
                        case 'v': to_sort_souths.add(new Coordinate(j,i)); break;
                    }
                }
            }

            easts = new HashSet<>(); inactive_easts = new HashSet<>();
            for (Coordinate position : to_sort_easts) {
                if (occupied(position.add(east_movement), to_sort_easts, to_sort_souths))
                    inactive_easts.add(position);
                else
                    easts.add(position);
            }

            souths = new HashSet<>(); inactive_souths = new HashSet<>();
            for (Coordinate position : to_sort_souths) {
                if (occupied(position.add(south_movement), to_sort_easts, to_sort_souths))
                    inactive_souths.add(position);
                else
                    souths.add(position);
            }
        }

        private Coordinate wrap(Coordinate position) {
            if (position.x < 0)
                return new Coordinate(right - 1, position.y);
            if (right <= position.x)
                return new Coordinate(0, position.y);
            if (position.y < 0)
                return new Coordinate(position.x, bottom - 1);
            if (bottom <= position.y)
                return new Coordinate(position.x, 0);
            return position;
        }

        @SafeVarargs
        private boolean occupied(Coordinate destination, HashSet<Coordinate>... to_check) {
            destination = wrap(destination);
            for (HashSet<Coordinate> subcheck : to_check)
                if (subcheck.contains(destination)) return true;
            return false;
        }
        private boolean occupied(Coordinate destination) {
            return occupied(destination, easts, souths, inactive_easts, inactive_souths);
        }

        private void vacate(Coordinate position) {
            Coordinate previous;
            previous = wrap(position.sub(east_movement));
            if (inactive_easts.contains(previous)) {
                inactive_easts.remove(previous);
                easts.add(previous);
            }
            
            previous = wrap(position.sub(south_movement));
            if (inactive_souths.contains(previous)) {
                inactive_souths.remove(previous);
                souths.add(previous);
            }
        }

        private boolean step() {
            HashSet<Coordinate> next_easts = new HashSet<>(), vacated = new HashSet<>();
            for (Coordinate cucumber : easts) {
                Coordinate destination = cucumber.add(east_movement);
                if (occupied(destination)) {
                    inactive_easts.add(cucumber);
                    continue;
                }
                vacated.add(cucumber);
                next_easts.add(wrap(destination));
            }
            easts = next_easts;
            for (Coordinate moved : vacated)
                vacate(moved);
            
            HashSet<Coordinate> next_souths = new HashSet<>(); vacated.clear();
            for (Coordinate cucumber : souths) {
                Coordinate destination = cucumber.add(south_movement);
                if (occupied(destination)) {
                    inactive_souths.add(cucumber);
                    continue;
                }
                vacated.add(cucumber);
                next_souths.add(wrap(destination));
            }
            souths = next_souths;
            for (Coordinate moved : vacated)
                vacate(moved);

            for (Iterator<Coordinate> iter = easts.iterator(); iter.hasNext();) {
                Coordinate next = iter.next();
                if (!occupied(next.add(east_movement))) continue;
                iter.remove();
                inactive_easts.add(next);
            }
            for (Iterator<Coordinate> iter = souths.iterator(); iter.hasNext();) {
                Coordinate next = iter.next();
                if (!occupied(next.add(south_movement))) continue;
                iter.remove();
                inactive_souths.add(next);
            }
            return easts.size() > 0 || souths.size() > 0;
        }

        public int count_steps() {
            int i = 2;
            while (step()) i++;
            return i;
        }

        public String toString() {
            StringBuilder result = new StringBuilder();
            for (int i = 0; i < bottom; i++) {
                for (int j = 0; j < right; j++) {
                    Coordinate position = new Coordinate(j,i);
                    if (!occupied(position)) {
                        result.append('.');
                        continue;
                    }
                    if (occupied(position, souths, inactive_souths)) {
                        result.append('v');
                        continue;
                    } else {
                        result.append('>');
                    }
                }
                result.append('\n');
            }
            return result.toString();
        }
    }

    public static void main(String[] args) {
        try {
            List<String> raw_data = Files
                    .lines(Paths.get("2021/day25/day25.txt"))
                    .toList();
            CucumberSwarm part_1 = new CucumberSwarm(raw_data);
            System.out.println(part_1.count_steps());
            
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
