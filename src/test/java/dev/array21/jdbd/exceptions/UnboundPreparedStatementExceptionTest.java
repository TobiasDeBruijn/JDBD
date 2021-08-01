package dev.array21.jdbd.exceptions;

import static org.junit.Assert.assertEquals;

import org.junit.Test;

public class UnboundPreparedStatementExceptionTest {

	@Test
	public void testEmptyConstructor() {
		assertEquals(null, new UnboundPreparedStatementException().getMessage());
	}
	
	@Test
	public void testMessageConstructor() {
		assertEquals("foo", new UnboundPreparedStatementException("foo").getMessage());
	}
}
