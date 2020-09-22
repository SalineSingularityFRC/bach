//
//  This Source Code Form is subject to the terms of the Mozilla Public
//  License, v. 2.0. If a copy of the MPL was not distributed with this
//  file, You can obtain one at http://mozilla.org/MPL/2.0/.
//

import java.io.File;
import java.io.BufferedReader;
import java.io.FileReader;
import java.util.regex.Matcher;
import java.util.regex.Pattern;
import java.util.ArrayList;

public class Bach {
	final static Pattern pattern = Pattern.compile("[\\s]*///[\\s]*");

	/// Walk a directory and scan each file for doc comments
	/// Takes in a File f to read from
	static ArrayList<ArrayList<String>> walk(final File f) {
		ArrayList<ArrayList<String>> comments = new ArrayList();
		comments.add(new ArrayList());
		int idx = 0;

		for (final File entry : f.listFiles()) {
			// If the file is a directory, recurse to scan that directory
			if (entry.isDirectory()) {
				walk(entry);
			} else {
				try (BufferedReader br = new BufferedReader(new FileReader(entry))) {
					String line;
					while ((line = br.readLine()) != null) {
						if (pattern.matcher(line).find()) {
							// Mark the current index, skip 1, read the next entry
							// Essentially a peek() in other languages, but probably unsafe
							br.mark(0);
							br.skip(1);
							String next = br.readLine();
							br.reset();

							if (pattern.matcher(next).find()) {
								comments.get(idx).add(line);
							} else {
								// Create a new field of comments
								comments.get(idx).add(line);
								comments.get(idx).add(next);
								comments.add(new ArrayList());
								idx += 1;
							}
						}
					}
				} catch (Exception e) {
					System.out.println(e);
				}
			}
		}

		return comments;
	}

	/// Main fn
	public static void main(String[] args) {
		// Make an output directory
		new File("./bach").mkdirs();
		
		ArrayList<ArrayList<String>> comments = walk(new File("."));
		//System.out.println(comments);
	}
}
