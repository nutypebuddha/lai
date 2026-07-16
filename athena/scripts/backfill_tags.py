#!/usr/bin/env python3
"""Backfill tags for all 80 formulas across TOML files."""

import os
import re

TOML_DIR = os.path.join(os.path.dirname(__file__), "..", "formulas")

TAGS = {
    # Aries - Math
    "add":            ["arithmetic", "basic", "operation"],
    "subtract":       ["arithmetic", "basic", "operation"],
    "multiply":       ["arithmetic", "basic", "operation"],
    "divide":         ["arithmetic", "basic", "operation"],
    "pythagorean":    ["geometry", "triangle", "theorem"],
    "mean":           ["statistics", "average"],
    "abs_diff":       ["arithmetic", "distance", "absolute"],
    "derivative_power_rule": ["calculus", "differentiation"],
    "integral_power_rule":   ["calculus", "integration"],

    # Taurus - Physics
    "speed":          ["motion", "kinematics"],
    "density":        ["matter", "properties"],
    "newtons_second": ["mechanics", "force", "newton"],
    "momentum":       ["mechanics", "conservation"],
    "kinetic_energy": ["energy", "mechanics"],
    "potential_energy": ["energy", "gravity"],
    "acceleration":   ["mechanics", "dynamics"],

    # Gemini - Astronomy
    "escape_velocity":  ["astronomy", "gravity", "space"],
    "orbital_velocity": ["astronomy", "gravity", "orbit"],
    "light_travel_time": ["astronomy", "distance", "light"],

    # Cancer - Earth
    "temp_conversion_c_to_f": ["temperature", "conversion"],
    "temp_conversion_f_to_c": ["temperature", "conversion"],
    "pressure_depth":        ["water", "pressure", "ocean"],
    "wind_chill":            ["weather", "temperature", "cold"],

    # Leo - Biology
    "bmi":                ["health", "body", "mass"],
    "heart_rate_reserve": ["heart", "fitness", "exercise"],
    "max_heart_rate":     ["heart", "age", "fitness"],
    "bmi_category":       ["health", "weight", "classification"],

    # Virgo - Economics
    "simple_interest": ["finance", "interest"],
    "profit_margin":   ["business", "profitability"],
    "discount_price":  ["shopping", "pricing"],
    "tax_amount":      ["tax", "pricing"],

    # Libra - Engineering
    "ohms_law":       ["electricity", "circuits"],
    "power_electric": ["electricity", "power"],
    "stress":         ["mechanics", "materials"],
    "lever":          ["mechanics", "simple_machine"],

    # Scorpio - CS
    "binary_and":       ["logic", "gates", "binary"],
    "binary_or":        ["logic", "gates", "binary"],
    "perceptron":       ["machine_learning", "neural_network"],
    "accuracy":         ["evaluation", "classification"],
    "f1_score":         ["evaluation", "metrics"],
    "time_complexity":  ["algorithms", "complexity"],
    "entropy":          ["information", "uncertainty", "compression"],

    # Sagittarius - History
    "generation_gap": ["time", "genealogy"],
    "carbon_dating":  ["archaeology", "dating", "radioactive"],

    # Capricorn - Language
    "sentence_length":      ["readability", "writing"],
    "flesch_reading_ease":  ["readability", "linguistics"],
    "type_token_ratio":     ["linguistics", "diversity"],
    "lexical_density":      ["linguistics", "analysis"],

    # Aquarius - Philosophy
    "utility":               ["ethics", "utilitarianism"],
    "categorical_imperative": ["ethics", "kant", "deontology"],

    # Pisces - Psychology
    "arousal_valence":          ["emotion", "affect"],
    "reaction_time":            ["cognition", "response"],
    "forgetting_curve":         ["memory", "learning"],
    "psychosis_severity":       ["clinical", "psychosis"],
    "affective_instability_index": ["mood", "clinical"],
    "clinical_insight_score":   ["clinical", "assessment"],

    # Bridging: Astrology Knowledge
    "planetary_domain_resonance": ["astrology", "logic"],
    "venusian_harmony":           ["design", "harmony"],
    "mercurial_transmission":     ["communication", "linguistics"],
    "lunar_cycle_learning":       ["learning", "emotion"],
    "solar_creative_power":       ["creativity", "mathematics"],
    "jovian_expansion":           ["artificial_intelligence", "wisdom"],
    "saturnine_structure":        ["economics", "stability"],
    "martian_healing":            ["health", "regeneration"],
    "saturnine_philosophy":       ["philosophy", "discipline"],
    "lunar_psychology":           ["psychology", "emotion", "depth"],

    # Bridging: CS/Math
    "precision_recall_fusion":  ["machine_learning", "evaluation"],
    "computational_complexity": ["algorithms", "scalability"],
    "ai_ethic_bridge":          ["artificial_intelligence", "ethics"],

    # Bridging: Environment/Bio
    "population_growth": ["population", "ecology"],
    "eco_footprint":     ["ecology", "sustainability"],
    "climate_economics": ["climate", "economics"],

    # Bridging: Language/Psych
    "sentiment_analysis":    ["nlp", "emotion"],
    "reading_comprehension": ["linguistics", "education"],
    "moral_psychology":      ["ethics", "psychology"],

    # Bridging: Physics/Engineering
    "momentum_to_ke":  ["mechanics", "energy"],
    "force_to_power":  ["mechanics", "power"],
    "work_energy":     ["mechanics", "energy"],
    "math_physics_bridge": ["mathematics", "physics"],
    "bio_chem_bridge":     ["biology", "chemistry"],
}


def add_tags_to_entry(match):
    """Callback to add tags after the id line in a [[formula]] block."""
    block = match.group(0)
    # Check if tags already present
    if "tags =" in block:
        return block
    # Extract formula id
    id_match = re.search(r'^id\s*=\s*"([^"]+)"', block, re.MULTILINE)
    if not id_match:
        return block
    fid = id_match.group(1)
    tags = TAGS.get(fid)
    if not tags:
        print(f"  WARNING: no tags defined for '{fid}'")
        return block
    # Insert tags line after the id line
    tag_line = 'tags = ["' + '", "'.join(tags) + '"]'
    # Find the id line position relative to block start
    block_lines = block.split('\n')
    new_lines = []
    inserted = False
    for line in block_lines:
        new_lines.append(line)
        if line.strip().startswith('id =') and not inserted:
            new_lines.append(tag_line)
            inserted = True
    return '\n'.join(new_lines)


def process_file(filepath):
    """Add tags to all [[formula]] entries in a TOML file."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Match each [[formula]] block (up to next [[ or end of file)
    pattern = r'\[\[formula\]\](?:.*?)(?=\n\[\[|\Z)'
    new_content = re.sub(pattern, add_tags_to_entry, content, flags=re.DOTALL)
    
    if new_content != content:
        with open(filepath, 'w') as f:
            f.write(new_content)
        return True
    return False


def main():
    modified = 0
    for root, dirs, files in os.walk(TOML_DIR):
        for fname in sorted(files):
            if fname.endswith('.toml'):
                fpath = os.path.join(root, fname)
                print(f"Processing: {os.path.relpath(fpath, TOML_DIR)}")
                if process_file(fpath):
                    modified += 1
                    print(f"  -> modified")
    print(f"\nDone. Modified {modified} file(s).")


if __name__ == '__main__':
    main()
