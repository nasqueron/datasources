from nasqueron_datasources.pipelines import commands
import unittest


class TestCommands(unittest.TestCase):
    def test_parse_environment(self):
        expected = {
            "FOO": "This is a sentence.",
            "QUUX": "666",  # everything is parsed as a string
            "BAR": "",  # an empty string is used instead of None for empty values
        }

        with open("files/env") as fd:
            self.assertDictEqual(expected, commands.parse_environment(fd))


if __name__ == "__main__":
    unittest.main()
