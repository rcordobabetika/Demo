package utilities;

import io.restassured.RestAssured;
import io.restassured.builder.RequestSpecBuilder;
import io.restassured.http.ContentType;
import io.restassured.specification.RequestSpecification;

import java.net.URI;

import java.net.URISyntaxException;
import java.util.Map;

public class RestAssuredExtension {

    public static RequestSpecification Request;

    public RestAssuredExtension(){
        //Arrange
        RequestSpecBuilder builder = new RequestSpecBuilder();
        builder.setBaseUri("http://localhost:3000");
        builder.setContentType(ContentType.JSON);
        var requestSpec = builder.build();
        Request = RestAssured.given().spec(requestSpec);
    }

    public static void GetOpsWithPathParameter (String url, Map<String, String> pathParams) throws URISyntaxException {
        //Act
        Request.pathParams(pathParams);
        Request.get(new URI(url));
    }
    public static void GetOps (String url) throws URISyntaxException {
        //Act
        Request.get(new URI(url));
    }

}