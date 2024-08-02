
import java.io.File;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class day21 {
    public static int part_1(int p1_position, int p2_position) {
        int p1_score = 0, p2_score = 0, dice_rolls = 0, roll;
        p1_position--; p2_position--;
        while (Math.max(p1_score, p2_score) < 1000) {
            roll = 6 + 9 * dice_rolls; // simulates rolling 3 times on deterministic dice
            if (dice_rolls % 2 == 0) {
                p1_position = (p1_position + roll) % 10;
                p1_score += p1_position + 1;
            } else {
                p2_position = (p2_position + roll) % 10;
                p2_score += p2_position + 1;
            }
            dice_rolls++;
        }
        return 3 * dice_rolls * Math.min(p1_score, p2_score);
    }

    public static long part_2(int p1_position, int p2_position) {
        Map<Integer, Integer> futures = new HashMap<>();
        futures.put(3,1); futures.put(4,3); futures.put(5,6); futures.put(6,7); 
        futures.put(7,6); futures.put(8,3); futures.put(9,1);
        
        Map<List<Integer>, Long> matches = new HashMap<>();
        matches.put(Arrays.asList(p1_position - 1, 0, p2_position - 1, 0), 1l); // 1 position, 1 score, 2 position, 2 score
        long p1_wins = 0, p2_wins = 0;
        boolean player_1 = true;
        while (matches.size() > 0) {
            Map<List<Integer>, Long> next_matches = new HashMap<>();
            for (Map.Entry<List<Integer>, Long> match : matches.entrySet()) {
                if (match.getKey().get(1) >= 21) {
                    p1_wins += match.getValue();
                    continue;
                }
                if (match.getKey().get(3) >= 21) {
                    p2_wins += match.getValue();
                    continue;
                }

                for (Map.Entry<Integer, Integer> future : futures.entrySet()) {
                    List<Integer> next = new ArrayList<>(match.getKey());
                    if (player_1) {
                        next.set(0, (next.get(0) + future.getKey()) % 10);
                        next.set(1, next.get(1) + next.get(0) + 1);
                    } else {
                        next.set(2, (next.get(2) + future.getKey()) % 10);
                        next.set(3, next.get(3) + next.get(2) + 1);
                    }
                    
                    if (!next_matches.containsKey(next))
                        next_matches.put(next, 0l);
                    next_matches.put(next, next_matches.get(next) + match.getValue() * future.getValue()); 
                }
            }
            matches = next_matches;
            player_1 = !player_1;
        }
        return Math.max(p1_wins, p2_wins);
    }

    public static void main(String[] args) {
        try {
            Scanner input_scanner = new Scanner(new File("2021/day21/day21.txt"));
            Pattern reader = Pattern.compile("^Player \\d starting position: (\\d+)$");

            int player_1_start, player_2_start;
            Matcher line = reader.matcher(input_scanner.nextLine());
            line.find();
            player_1_start = Integer.parseInt(line.group(1));
            line = reader.matcher(input_scanner.nextLine());
            line.find();
            player_2_start = Integer.parseInt(line.group(1));
            input_scanner.close();

            System.out.println(part_1(player_1_start, player_2_start));
            System.out.println(part_2(player_1_start, player_2_start));
            
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
