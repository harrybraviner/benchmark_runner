package braviner.harry.benchmarking.java_reader;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * The purpose of this program is purely to count the number of lines in the file.
 */
public class SerialLineCounter {

    public static void main(String[] args) throws IOException {

        if (args.length != 1) {
            System.err.println("Usage: [input filename]");
            return;
        }

        String filename = args[0];

        try (BufferedReader reader = new BufferedReader(new FileReader(filename))) {

            AtomicInteger lineCounter = new AtomicInteger();

            long startTime = System.nanoTime();
            reader.lines().forEach(line -> lineCounter.incrementAndGet());
            long endTime = System.nanoTime();

            System.out.println("Read " + lineCounter.get() + " lines");
            long nanosElapsed = endTime - startTime;
            double secondsElapsed = nanosElapsed / 1e9;
            System.out.println("Took " + secondsElapsed + " seconds");
        }
    }
}
