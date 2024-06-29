
import java.io.File;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Scanner;
import java.util.stream.Collectors;

public class day14 {
    public static void count_after_N_grows(Map<String, Character> insertion_rules, String template, int n) {
        // convert template to map of polymer pair counts
        Map<String, Long> polymer_pairs = new HashMap<>();
        for (int index = 0; index < template.length() - 1; index++)
            polymer_pairs.put(template.substring(index, index+2), polymer_pairs.getOrDefault(template.substring(index, index+2),0l)+1);
        // convert template to map of element counts
        Map<Character, Long> element_counts = new HashMap<>();
        for (char c : template.toCharArray())
            element_counts.put(c, element_counts.getOrDefault(c, 0l) + 1);

        for (int index = 0; index < n; index++) {
            Map<String, Long> next_polymer_pairs = new HashMap<>();
            for (Map.Entry<String, Long> pair : polymer_pairs.entrySet()) {
                // identify the inserted character, get the resultant element pairs from the split
                char inserted = insertion_rules.get(pair.getKey());
                String  left  = String.format("%c%c", pair.getKey().charAt(0), inserted),
                        right = String.format("%c%c", inserted, pair.getKey().charAt(1));

                // increment the left pair, right pair, and element count
                next_polymer_pairs.put(left , next_polymer_pairs.getOrDefault(left , 0l) + pair.getValue());
                next_polymer_pairs.put(right, next_polymer_pairs.getOrDefault(right, 0l) + pair.getValue());
                element_counts.put(inserted, element_counts.getOrDefault(inserted, 0l) + pair.getValue());
            }
            polymer_pairs = next_polymer_pairs;
        }

        char    max_char = Collections.max(element_counts.entrySet(), Map.Entry.comparingByValue()).getKey(),
                min_char = Collections.min(element_counts.entrySet(), Map.Entry.comparingByValue()).getKey();
        System.out.println(element_counts.get(max_char) - element_counts.get(min_char));
    }
    
    public static void main(String[] args) {
        try {
            Scanner input_scanner = new Scanner(new File("2021/day14/day14.txt"));
            input_scanner.useDelimiter("\n\n");

            String data_line = input_scanner.next();
            Map<String, Character> insertions = Arrays.stream(input_scanner.next().split("\n"))
                .map(x -> x.split(" -> "))
                .collect(Collectors.toMap(x -> x[0], x -> x[1].charAt(0)));
            input_scanner.close();

            count_after_N_grows(insertions, data_line, 10);
            count_after_N_grows(insertions, data_line, 40);
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
