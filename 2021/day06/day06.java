
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

            Map<Integer, Integer> fish_ages = new HashMap<>();
            for (int fish : fish_input) {
                if (!fish_ages.containsKey(fish))
                    fish_ages.put(fish, 0);
                fish_ages.put(fish, fish_ages.get(fish) + 1);
            }

            int max_cycles = 80, fish_spawn_cycle = 7, fish_new_buffer = 2;
            for (int i = 0; i < max_cycles; i++) {
                int next_fish_cycle = i + fish_spawn_cycle, new_fish_cycle = i + fish_spawn_cycle + fish_new_buffer;
                int this_fish_count = fish_ages.getOrDefault(i, 0);
                fish_ages.put(next_fish_cycle, this_fish_count + fish_ages.getOrDefault(next_fish_cycle, 0));
                fish_ages.put(new_fish_cycle, this_fish_count + fish_ages.getOrDefault(new_fish_cycle, 0));
            }
            int fish_count = fish_ages.entrySet().stream().mapToInt(x -> x.getKey() >= max_cycles ? x.getValue() : 0).sum();
            System.out.println(fish_count);
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
