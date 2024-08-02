import java.io.File;
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
            
        } catch (Exception e) {
            System.out.println(e.toString());
        }
    }
}
