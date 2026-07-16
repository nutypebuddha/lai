# Athena Python Client
# 
# Bullet-train integration for LLMs: deterministic reasoning as a tool.
# 
# Usage:
#   import athena
#   client = athena.Client()
#   result = client.solve("mars newtons_second")
#   print(result.value, result.confidence)

import json
import subprocess
import shlex
from dataclasses import dataclass
from typing import Optional, List, Dict, Any
from pathlib import Path


@dataclass
class AthenaSolveResult:
    """Structured result from Athena solve."""
    query: str
    success: bool
    confidence: float
    value: Optional[float]
    formula: Optional[str]
    domain: Optional[str]
    domain_aligned: Optional[bool]
    summary: str
    raw_output: str


@dataclass
class AthenaEntity:
    """Entity information."""
    id: str
    name: str
    domain: str
    properties: Dict[str, float]
    constants: Dict[str, float]
    description: str


class AthenaClient:
    """
    Athena Python client - deterministic reasoning for LLMs.
    
    Use as a tool: LLM calls solve() → Athena returns exact result + confidence.
    
    Example:
        client = AthenaClient()
        result = client.solve("mars newtons_second")
        # result.value = 2.387e27 (force on Mars in Newtons)
        # result.confidence = 1.0
        # result.domain_aligned = False  # Aries entity vs Taurus formula
    """
    
    def __init__(
        self,
        binary_path: str = "/root/athena/target/release/athena",
        timeout: float = 30.0,
        env: Optional[Dict[str, str]] = None
    ):
        self.binary_path = binary_path
        self.timeout = timeout
        self.env = env or {}
    
    def _run(self, args: list[str]) -> str:
        """Run athena CLI and return combined output."""
        cmd = [self.binary_path] + args
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            timeout=self.timeout,
            env={**os.environ, **self.env}
        )
        combined = result.stdout + "\n" + result.stderr
        if result.returncode != 0 and "Validation:" not in combined:
            raise RuntimeError(f"Athena CLI failed: {result.stderr}")
        return combined
    
    def solve(self, query: str) -> AthenaSolveResult:
        """
        Natural language solve - auto-resolves entities, fills constants.
        
        Args:
            query: Natural language query like "mars newtons_second" or 
                   "force mass=5 acceleration=9.8"
        
        Returns:
            AthenaSolveResult with value, confidence, and metadata.
        """
        output = self._run(["solve", query])
        return self._parse_solve_output(query, output)
    
    def _parse_solve_output(self, query: str, output: str) -> AthenaSolveResult:
        """Parse Athena solve CLI output."""
        # Skip entity loading logs - find the actual result section
        lines = output.strip().split('\n')
        # Find the "=== Bankai Solve ===" line and parse from there
        start_idx = 0
        for i, line in enumerate(lines):
            if "=== Bankai Solve ===" in line:
                start_idx = i
                break
        
        summary = ""
        value = None
        confidence = 0.0
        success = False
        formula = None
        domain = None
        domain_aligned = None
        
        for line in lines[start_idx:]:
            if line.startswith("Bankai: "):
                summary = line
                # Parse: Bankai: formula → output = value (domain: X) [domain mismatch]
                if " → " in line:
                    parts = line.split(" → ")
                    formula = parts[0].replace("Bankai: ", "").split()[0]
                    rest = parts[1]
                    if " = " in rest:
                        val_str = rest.split(" = ")[1].split()[0]
                        try:
                            value = float(val_str)
                        except ValueError:
                            pass
                if "(domain: " in line:
                    domain = line.split("(domain: ")[1].split(")")[0]
                if "[domain mismatch]" in line:
                    domain_aligned = False
                else:
                    domain_aligned = True
            elif line.startswith("Validation: "):
                if "confidence:" in line:
                    try:
                        confidence = float(line.split("confidence:")[1].split(")")[0].strip())
                    except (ValueError, IndexError):
                        pass
                if "Passed" in line:
                    success = True
                elif "Failed" in line:
                    success = False
        
        return AthenaSolveResult(
            query=query,
            success=success,
            confidence=confidence,
            value=value,
            formula=formula,
            domain=domain,
            domain_aligned=domain_aligned,
            summary=summary or "unknown",
            raw_output=output
        )
    
    def entity_eval(
        self, 
        formula: str, 
        entity: str, 
        args: Optional[Dict[str, float]] = None
    ) -> AthenaSolveResult:
        """
        Evaluate a formula grounded in a specific entity.
        Auto-fills missing args from entity properties/constants.
        """
        cmd = ["entity-eval", "--formula", formula, "--entity", entity]
        if args:
            for k, v in args.items():
                cmd.extend(["--args", f"{k}={v}"])
        output = self._run(cmd)
        return self._parse_entity_eval_output(f"{entity} {formula}", output)
    
    def _parse_entity_eval_output(self, query: str, output: str) -> AthenaSolveResult:
        """Parse Athena entity-eval CLI output."""
        lines = output.strip().split('\n')
        value = None
        confidence = 1.0
        success = False
        formula = None
        entity_name = None
        domain = None
        domain_aligned = None
        
        for line in lines:
            if ' → ' in line and 'Entity:' not in line and 'Args:' not in line:
                parts = line.split(' → ')
                formula = parts[0].strip()
                val_str = parts[1].split()[0]
                try:
                    value = float(val_str)
                    success = True
                except ValueError:
                    pass
            elif line.startswith('Entity: '):
                entity_name = line.split('Entity: ')[1].split(' (')[0]
                domain = line.split('(')[1].split(')')[0]
            elif 'domain mismatch' in line.lower() or 'mismatch' in line.lower():
                domain_aligned = False
            elif '(Warning:' in line and 'domain mismatch' in line.lower():
                domain_aligned = False
            elif 'Entity:' in line and domain_aligned is None:
                # Default to aligned unless mismatch detected
                domain_aligned = True
        
        return AthenaSolveResult(
            query=query,
            success=success,
            confidence=confidence,
            value=value,
            formula=formula,
            domain=domain,
            domain_aligned=domain_aligned,
            summary=f"{formula} → {value}",
            raw_output=output
        )
    
    def validate(self, expression: str, gate: str = "math") -> Dict[str, Any]:
        """Validate an expression through specified gate(s)."""
        cmd = ["validate", expression, "--gate", gate]
        output = self._run(cmd)
        return {"expression": expression, "gate": gate, "raw": output}
    
    def reason(self, have: List[str], want: str, execute: bool = False, args: Optional[Dict[str, float]] = None) -> Dict[str, Any]:
        """Find reasoning path from have → want, optionally execute."""
        cmd = ["reason", "--have", ",".join(have), "--want", want]
        if execute:
            cmd.append("--execute")
        if args:
            for k, v in args.items():
                cmd.extend(["--args", f"{k}={v}"])
        output = self._run(cmd)
        return {"have": have, "want": want, "raw": output}
    
    def search(self, keyword: str) -> Dict[str, Any]:
        """Search formulas by keyword."""
        output = self._run(["search", keyword])
        return {"keyword": keyword, "raw": output}
    
    def entity_aspect(self, from_entity: str, to_entity: str) -> Dict[str, Any]:
        """Compute aspect relationship between two entities."""
        output = self._run(["entity-aspect", "--from", from_entity, "--to", to_entity])
        return {"from": from_entity, "to": to_entity, "raw": output}
    
    def entity_get(self, entity_id: str) -> Optional[AthenaEntity]:
        """Get entity details."""
        output = self._run(["entity-get", "--id", entity_id])
        return self._parse_entity_output(entity_id, output)
    
    def _parse_entity_output(self, entity_id: str, output: str) -> Optional[AthenaEntity]:
        """Parse entity-get output."""
        lines = output.strip().split('\n')
        in_props = False
        in_consts = False
        props = {}
        consts = {}
        name = ""
        domain = ""
        desc = ""
        
        for line in lines:
            if line.startswith("=== Entity: "):
                name = line.replace("=== Entity: ", "").strip()
            elif line.startswith("  Domain:"):
                domain = line.replace("  Domain:", "").strip()
            elif line.startswith("  Description:"):
                desc = line.replace("  Description:", "").strip()
            elif line.startswith("  Properties:"):
                in_props = True
                in_consts = False
            elif line.startswith("  Constants:") or line.startswith("  Relationships:"):
                in_props = False
            elif in_props and "=" in line:
                parts = line.strip().split("=")
                if len(parts) == 2:
                    try:
                        props[parts[0].strip()] = float(parts[1].strip())
                    except ValueError:
                        pass
        
        return AthenaEntity(
            id=entity_id,
            name=name,
            domain=domain,
            properties=props,
            constants=consts,
            description=desc
        )


import os  # moved to top in production

# Convenience function for quick use
def solve(query: str, binary: str = "/root/athena/target/release/athena") -> AthenaSolveResult:
    """Quick one-liner solve."""
    return AthenaClient(binary_path=binary).solve(query)


if __name__ == "__main__":
    # Demo
    client = AthenaClient()
    
    # Auto-fills Mars constants (mass_kg, surface_gravity_ms2)
    result = client.solve("mars newtons_second")
    print(f"Mars F=ma: {result.value:.3e} N (confidence: {result.confidence:.2f})")
    print(f"  Domain aligned: {result.domain_aligned}")
    print(f"  Formula: {result.formula}")
    
    # Entity eval with partial args
    result2 = client.entity_eval("newtons_second", "venus", {"acceleration": 8.87})
    print(f"\nVenus F=ma (partial args): {result2.value:.3e} N")
    
    # Cross-domain chain
    result3 = client.reason(["mass", "velocity"], "work", execute=True, args={"mass": 2, "velocity": 3, "distance": 4})
    print(f"\nChain mass+velocity→work: {result3['raw']}")
    
    # Validation
    val = client.validate("2 + 2 = 4")
    print(f"\nValidation: {val['raw']}")
    
    # Entity aspect
    aspect = client.entity_aspect("mars", "venus")
    print(f"\nMars-Venus aspect: {aspect['raw']}")