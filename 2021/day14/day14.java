
import java.io.File;
import java.util.Arrays;
import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Scanner;
import java.util.stream.Collectors;

public class day14 {
    public static String step(Map<String, Character> insertions, String data_line_in) {
        StringBuilder data_line_out = new StringBuilder();
        for (int index = 0; index < data_line_in.length()-1; index++) {
            data_line_out.append(data_line_in.charAt(index));
            data_line_out.append(insertions.get(data_line_in.substring(index, index+2)));
        }
        data_line_out.append(data_line_in.charAt(data_line_in.length()-1));
        return data_line_out.toString();
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

            for (int index = 1; index <= 10; index++) {
                data_line = step(insertions, data_line);
            }

            Map<Character, Integer> element_counts = new HashMap<>();
            for (char c : data_line.toCharArray()) {
                element_counts.put(c, element_counts.getOrDefault(c, 0) + 1);
            }
            char    max_char = Collections.max(element_counts.entrySet(), Map.Entry.comparingByValue()).getKey(),
                    min_char = Collections.min(element_counts.entrySet(), Map.Entry.comparingByValue()).getKey();
            System.out.println(element_counts.get(max_char) - element_counts.get(min_char));

        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
