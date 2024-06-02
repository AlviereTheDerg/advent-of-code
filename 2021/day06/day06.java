
import java.io.File;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Scanner;

public class day06 {

    public static void main(String[] args) {
        try {
            File input_file = new File("2021/day06/day06.txt");
            Scanner input_scanner = new Scanner(input_file);
            List<Integer> fish_input = Arrays.stream(input_scanner.next().split(",")).map(Integer::parseInt).toList();
            input_scanner.close();

            Map<Integer, Long> fish_ages = new HashMap<>();
            for (int fish : fish_input) {
                if (!fish_ages.containsKey(fish))
                    fish_ages.put(fish, 0l);
                fish_ages.put(fish, fish_ages.get(fish) + 1);
            }

            int pt1_cycles = 80, fish_spawn_cycle = 7, fish_new_buffer = 2;
            for (int i = 0; i < pt1_cycles; i++) {
                int next_fish_cycle = i + fish_spawn_cycle, new_fish_cycle = i + fish_spawn_cycle + fish_new_buffer;
                long this_fish_count = fish_ages.getOrDefault(i, 0l);
                fish_ages.put(next_fish_cycle, this_fish_count + fish_ages.getOrDefault(next_fish_cycle, 0l));
                fish_ages.put(new_fish_cycle, this_fish_count + fish_ages.getOrDefault(new_fish_cycle, 0l));
            }
            System.out.println(fish_ages.entrySet().stream().mapToLong(x -> x.getKey() >= pt1_cycles ? x.getValue() : 0).sum());

            int pt2_cycles = 256;
            for (int i = pt1_cycles; i < pt2_cycles; i++) {
                int next_fish_cycle = i + fish_spawn_cycle, new_fish_cycle = i + fish_spawn_cycle + fish_new_buffer;
                long this_fish_count = fish_ages.getOrDefault(i, 0l);
                fish_ages.put(next_fish_cycle, this_fish_count + fish_ages.getOrDefault(next_fish_cycle, 0l));
                fish_ages.put(new_fish_cycle, this_fish_count + fish_ages.getOrDefault(new_fish_cycle, 0l));
            }
            System.out.println(fish_ages.entrySet().stream().mapToLong(x -> x.getKey() >= pt2_cycles ? x.getValue() : 0).sum());
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
