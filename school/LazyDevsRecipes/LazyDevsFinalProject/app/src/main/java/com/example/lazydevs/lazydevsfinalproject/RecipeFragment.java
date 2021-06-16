package com.example.lazydevs.lazydevsfinalproject;

import android.content.Intent;
import android.graphics.Color;
import android.net.Uri;
import android.os.Bundle;
import android.support.v4.app.Fragment;
import android.support.v4.app.FragmentTransaction;
import android.util.Log;
import android.view.LayoutInflater;
import android.view.View;
import android.view.ViewGroup;
import android.widget.ArrayAdapter;
import android.widget.Button;
import android.widget.ImageView;
import android.widget.ListView;
import android.widget.TextView;

import com.squareup.picasso.Picasso;
import org.json.JSONException;

public class RecipeFragment extends Fragment {
    private final String TAG = "Recipe Fragment";

    @Override
    public View onCreateView(LayoutInflater inflater, ViewGroup container,
                             Bundle savedInstanceState) {

        AppState state = AppState.getInstance();
        // Inflate the layout for this fragment

        Recipe recipe = null;
        try {
            recipe = state.makeRecipe();
        } catch (JSONException e) {
            e.printStackTrace();
        }

        // Set the Current title and image from the urls
        View view = inflater.inflate(R.layout.fragment_recipe, container, false);
        TextView title = (TextView)view.findViewById(R.id.recipeTitle);
        title.setText(recipe.getTitle());

        // Set the image of the recipe
        ImageView image = (ImageView)view.findViewById(R.id.recipeImage);
        Picasso.with(getContext()).load(state.getDishes().get(state.getPosition()).getImage()).into(image);

        // Set the url text and make it clickable
        final Uri uri = Uri.parse(recipe.getSourceURL());
        Button linkButton = (Button)view.findViewById(R.id.urlButton);
        linkButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                Intent intent = new Intent(Intent.ACTION_VIEW, uri);
                startActivity(intent);
            }
        });

        // Set the ingredients
        String[] ingredients = new String[0];
        try {
            ingredients = recipe.getIngredients();
        } catch (JSONException e) {
            e.printStackTrace();
        }

        // Show ingredients
        ListView listView = (ListView)view.findViewById(R.id.ingredientsListView);
        listView.setBackgroundColor(Color.parseColor("#4846ba"));
        ArrayAdapter<String> adapter = new ArrayAdapter<String>(getActivity().getApplicationContext(),
                android.R.layout.simple_list_item_1,
                android.R.id.text1,
                ingredients);

        listView.setAdapter(adapter);

        // Set the button for the steps
        Button stepsButton = (Button)view.findViewById(R.id.stepsButton);
        stepsButton.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View v) {
                StepsFragment stepsFrag = new StepsFragment();
                // Intialize transition to answers fragment
                FragmentTransaction transaction = getFragmentManager().beginTransaction();
                transaction.replace(R.id.recipe_fragment_frame, stepsFrag);
                transaction.addToBackStack(null);
                transaction.commit();
            }
        });

        Log.i(TAG, "Successfully Reached topic overview fragment");
        return view;
    }

    @Override
    public void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        Log.e(TAG, "The " + TAG + " for application has been created");
    }

    @Override
    public void onDestroy() {
        super.onDestroy();
        Log.e(TAG, "The " + TAG + " for application has been destroyed");
    }

    @Override
    public void onPause() {
        super.onPause();
        Log.e(TAG, "The " + TAG + " for application has been paused");
    }

    @Override
    public void onStart() {
        super.onStart();
        Log.e(TAG, "The " + TAG + " for application has been started");
    }

    @Override
    public void onResume() {
        super.onResume();
        Log.e(TAG, "The " + TAG + " for application has resumed");
    }

    @Override
    public void onStop() {
        super.onStop();
        Log.e(TAG, "The " + TAG + " for application has stopped");
    }
}
