
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class day23 {
    private static class Burrow {
        int[] hallway;
        List<List<Integer>> rooms;
        int[] steps;
        int room_size;

        public Burrow(String raw_input) {
            int[] values = raw_input.replaceAll("[#\\.\\s]", "").chars().map(x -> x - 'A' + 1).toArray();
            room_size = 2;

            hallway = new int[11]; // initializes to 0s
            steps = new int[4];

            rooms = new ArrayList<>();
            rooms.add(new ArrayList<>()); rooms.add(new ArrayList<>()); rooms.add(new ArrayList<>()); rooms.add(new ArrayList<>());
            rooms.get(0).add(values[4]); rooms.get(0).add(values[0]);
            rooms.get(1).add(values[5]); rooms.get(1).add(values[1]);
            rooms.get(2).add(values[6]); rooms.get(2).add(values[2]);
            rooms.get(3).add(values[7]); rooms.get(3).add(values[3]);
        }
        public Burrow(Burrow previous, int source, int destination) {
            // loading this as a copy of previous
            this.hallway = previous.hallway.clone();
            this.steps = previous.steps.clone();
            this.rooms = new ArrayList<>();
            this.room_size = previous.room_size;
            for (List<Integer> room : previous.rooms)
                this.rooms.add(new ArrayList<>(room));

            // performing move of entry in source to entry in destination (and updating the steps counter)
            // if source is a room, move it out to the hallway
            if (source % 2 == 0 && 2 <= source && source <= 8) {
                List<Integer> room = this.rooms.get(source / 2 - 1);
                int entry = room.remove(room.size() - 1);
                hallway[source] = entry;
                steps[entry - 1] += room_size - room.size();
            }

            // move around the hallway
            hallway[destination] = hallway[source];
            steps[hallway[source] - 1] += Math.abs(destination - source);
            hallway[source] = 0;

            // if destination is a room, move out of the hallway
            if (destination % 2 == 0 && 2 <= destination && destination <= 8) {
                List<Integer> room = this.rooms.get(destination / 2 - 1);
                room.add(hallway[destination]);
                steps[hallway[destination] - 1] += room_size + 1 - room.size();
                hallway[destination] = 0;
            }
        }

        private boolean path_open(int source, int destination) {
            int dir = source < destination ? -1 : 1;
            for (int i = destination; i != source; i += dir)
                if (hallway[i] != 0)
                    return false;
            return true;
        }
        public List<Burrow> get_potential_futures() {
            List<Burrow> futures = new ArrayList<>();

            // get futures that involve an amphipod coming out of a room and to the hallway
            for (int i = 0; i < 4; i++) {
                final int amphipod_type = i + 1;
                // if this room has no amphipods that should leave, then no amphipods should leave
                if (rooms.get(i).stream().filter(x -> x != amphipod_type).count() == 0) continue;
                for (int j = 0; j < hallway.length; j++) {
                    if (j % 2 == 0 && 2 <= j && j <= 8) continue; // skip room->room (instead room->hallway->room)
                    if (path_open((i+1)*2, j))
                        futures.add(new Burrow(this, (i+1)*2, j));
                }
            }

            // get futures that involve an amphipod going to a room from the hallway
            for (int i = 0; i < hallway.length; i++) {
                if (hallway[i] == 0) continue; // there isn't an amphipod here to move
                final int amphipod_type = hallway[i];
                // amphipod can only go in its room if there is a path and the room can be entered
                if (path_open(i, amphipod_type * 2) &&
                    rooms.get(amphipod_type - 1).stream().filter(x -> x != amphipod_type).count() == 0)
                        futures.add(new Burrow(this, i, amphipod_type*2));
            }

            return futures;
        }

        public boolean is_organized() {
            for (int i = 0; i < hallway.length; i++)
                if (hallway[i] != 0) return false; // if there's an amphipod in the hallway, burrow is not organized
            for (int i = 0; i < rooms.size(); i++) {
                final int amphipod_type = i + 1;
                if (rooms.get(i).stream().filter(x -> x != amphipod_type).count() != 0)
                    return false; // if a room has an amphipod it shouldn't, burrow is not organized
            }
            return true;
        }
        public int energy_used() {
            return 1 * steps[0] + 10 * steps[1] + 100 * steps[2] + 1000 * steps[3];
        }

        private char represent_amphipod(int amphipod) {
            return (amphipod == 0) ? '.' : (char) (amphipod - 1 + 'A');
        }
        private String stringify_row(int row) {
            return String.format("#%c#%c#%c#%c#",
            represent_amphipod(rooms.get(0).size() > row ? rooms.get(0).get(row) : 0),
            represent_amphipod(rooms.get(1).size() > row ? rooms.get(1).get(row) : 0),
            represent_amphipod(rooms.get(2).size() > row ? rooms.get(2).get(row) : 0),
            represent_amphipod(rooms.get(3).size() > row ? rooms.get(3).get(row) : 0)
            );
        }
        public String toString() {
            return String.format("#############\n#%s#\n##%s\n  #########",
                Arrays.stream(hallway).mapToObj(x -> represent_amphipod(x)).map(Object::toString).reduce("", String::concat),
                IntStream.range(0, room_size).map(x -> room_size - x).mapToObj(x -> stringify_row(x-1)).collect(Collectors.joining("\n  "))
            );
        }

        public boolean equals(Object obj) {
            if (!(obj instanceof Burrow)) return false;
            Burrow that = (Burrow) obj;
            for (int i = 0; i < hallway.length; i++)
                if (this.hallway[i] != that.hallway[i]) return false;
            return this.rooms.equals(that.rooms);
        }
        public int hashCode() {
            return this.toString().hashCode();
        }

        public void unfold() {
            room_size += 2;
            rooms.get(0).add(1, 4);
            rooms.get(0).add(2, 4);
            rooms.get(1).add(1, 2);
            rooms.get(1).add(2, 3);
            rooms.get(2).add(1, 1);
            rooms.get(2).add(2, 2);
            rooms.get(3).add(1, 3);
            rooms.get(3).add(2, 1);
        }
    }

    public static int find_minimum_energy(Burrow starting) {
        LinkedList<Burrow> search = new LinkedList<>();
        search.add(starting);
        int least_energy = Integer.MAX_VALUE;
        HashMap<Burrow, Integer> visiteds = new HashMap<>();
        while (search.size() > 0) {
            Burrow here = search.removeFirst();
            if (here.is_organized())
                least_energy = Math.min(least_energy, here.energy_used());
            else
                for (Burrow next : here.get_potential_futures()) {
                    // if that potential future has already been visited and that other visit used less energy, skip it
                    if (visiteds.containsKey(next) && next.energy_used() >= visiteds.get(next)) continue;

                    visiteds.put(next, next.energy_used());
                    search.add(next);
                }
        }
        return least_energy;
    }
    
    public static void main(String[] args) {
        try {
            Burrow starting = new Burrow(Files.lines(Paths.get("2021/day23/day23.txt")).reduce("", String::concat));
            System.out.println(find_minimum_energy(starting));
            starting.unfold();
            System.out.println(find_minimum_energy(starting));
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
